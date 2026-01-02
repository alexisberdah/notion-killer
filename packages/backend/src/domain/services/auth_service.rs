use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::JwtConfig,
    domain::entities::user::User,
    error::{AppError, Result},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
}

pub struct AuthService<'a> {
    db: &'a PgPool,
    jwt_config: &'a JwtConfig,
}

impl<'a> AuthService<'a> {
    pub fn new(db: &'a PgPool, jwt_config: &'a JwtConfig) -> Self {
        Self { db, jwt_config }
    }

    pub async fn user_exists(&self, email: &str) -> Result<bool> {
        let result = sqlx::query!(
            r#"SELECT id FROM users WHERE email = $1 AND deleted_at IS NULL"#,
            email
        )
        .fetch_optional(self.db)
        .await?;

        Ok(result.is_some())
    }

    pub async fn create_user(&self, email: &str, password: &str, name: &str) -> Result<User> {
        let password_hash = self.hash_password(password)?;
        let user_id = Uuid::new_v4();

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash, name)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, password_hash, name, avatar_url, email_verified, created_at, updated_at
            "#,
            user_id,
            email,
            password_hash,
            name
        )
        .fetch_one(self.db)
        .await?;

        // Create a default workspace for the user
        let workspace_id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO workspaces (id, name, owner_id, settings)
            VALUES ($1, $2, $3, '{}')
            "#,
            workspace_id,
            format!("{}'s Workspace", name),
            user_id
        )
        .execute(self.db)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO workspace_members (workspace_id, user_id, role, invited_by)
            VALUES ($1, $2, 'owner', $2)
            "#,
            workspace_id,
            user_id
        )
        .execute(self.db)
        .await?;

        Ok(user)
    }

    pub async fn verify_credentials(&self, email: &str, password: &str) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, name, avatar_url, email_verified, created_at, updated_at
            FROM users
            WHERE email = $1 AND deleted_at IS NULL
            "#,
            email
        )
        .fetch_optional(self.db)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

        let password_hash = user.password_hash.as_ref().ok_or(AppError::InvalidCredentials)?;

        self.verify_password(password, password_hash)?;

        Ok(user)
    }

    pub async fn generate_tokens(&self, user_id: Uuid) -> Result<(String, String)> {
        let access_token = self.create_token(user_id, "access", self.jwt_config.access_token_expiry)?;
        let refresh_token = self.create_token(user_id, "refresh", self.jwt_config.refresh_token_expiry)?;

        // Store refresh token hash in database
        let token_hash = self.hash_token(&refresh_token);
        sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            token_hash,
            Utc::now() + Duration::seconds(self.jwt_config.refresh_token_expiry)
        )
        .execute(self.db)
        .await?;

        Ok((access_token, refresh_token))
    }

    pub async fn refresh_tokens(&self, refresh_token: &str) -> Result<(User, String, String)> {
        // Verify token
        let claims = self.verify_token(refresh_token)?;

        if claims.token_type != "refresh" {
            return Err(AppError::InvalidToken);
        }

        let token_hash = self.hash_token(refresh_token);

        // Check if token exists and is not revoked
        let stored_token = sqlx::query!(
            r#"
            SELECT id, user_id FROM refresh_tokens
            WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > NOW()
            "#,
            token_hash
        )
        .fetch_optional(self.db)
        .await?
        .ok_or(AppError::InvalidToken)?;

        // Revoke old token
        sqlx::query!(
            r#"UPDATE refresh_tokens SET revoked_at = NOW() WHERE id = $1"#,
            stored_token.id
        )
        .execute(self.db)
        .await?;

        // Get user
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, name, avatar_url, email_verified, created_at, updated_at
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            stored_token.user_id
        )
        .fetch_one(self.db)
        .await?;

        // Generate new tokens
        let (access_token, new_refresh_token) = self.generate_tokens(user.id).await?;

        Ok((user, access_token, new_refresh_token))
    }

    pub async fn revoke_token(&self, refresh_token: &str) -> Result<()> {
        let token_hash = self.hash_token(refresh_token);

        sqlx::query!(
            r#"UPDATE refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1"#,
            token_hash
        )
        .execute(self.db)
        .await?;

        Ok(())
    }

    pub fn verify_access_token(&self, token: &str) -> Result<Uuid> {
        let claims = self.verify_token(token)?;

        if claims.token_type != "access" {
            return Err(AppError::InvalidToken);
        }

        Ok(claims.sub)
    }

    fn create_token(&self, user_id: Uuid, token_type: &str, expiry_seconds: i64) -> Result<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id,
            exp: (now + Duration::seconds(expiry_seconds)).timestamp(),
            iat: now.timestamp(),
            token_type: token_type.to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_config.secret.as_bytes()),
        )
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Failed to create token")))?;

        Ok(token)
    }

    fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_config.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::InvalidToken,
        })?;

        Ok(token_data.claims)
    }

    fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::Internal(anyhow::anyhow!("Failed to hash password")))?
            .to_string();

        Ok(password_hash)
    }

    fn verify_password(&self, password: &str, password_hash: &str) -> Result<()> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid password hash")))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::InvalidCredentials)?;

        Ok(())
    }

    fn hash_token(&self, token: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        token.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}
