use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::services::auth_service::AuthService,
    error::{AppError, Result},
    AppState,
};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>> {
    // Validate input
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let auth_service = AuthService::new(&state.db, &state.config.jwt);

    // Check if user already exists
    if auth_service.user_exists(&payload.email).await? {
        return Err(AppError::UserAlreadyExists);
    }

    // Create user
    let user = auth_service
        .create_user(&payload.email, &payload.password, &payload.name)
        .await?;

    // Generate tokens
    let (access_token, refresh_token) = auth_service.generate_tokens(user.id).await?;

    Ok(Json(AuthResponse {
        user: UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            avatar_url: user.avatar_url,
        },
        access_token,
        refresh_token,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let auth_service = AuthService::new(&state.db, &state.config.jwt);

    // Verify credentials
    let user = auth_service
        .verify_credentials(&payload.email, &payload.password)
        .await?;

    // Generate tokens
    let (access_token, refresh_token) = auth_service.generate_tokens(user.id).await?;

    Ok(Json(AuthResponse {
        user: UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            avatar_url: user.avatar_url,
        },
        access_token,
        refresh_token,
    }))
}

pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>> {
    let auth_service = AuthService::new(&state.db, &state.config.jwt);

    // Verify and refresh tokens
    let (user, access_token, refresh_token) = auth_service
        .refresh_tokens(&payload.refresh_token)
        .await?;

    Ok(Json(AuthResponse {
        user: UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            avatar_url: user.avatar_url,
        },
        access_token,
        refresh_token,
    }))
}

pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<serde_json::Value>> {
    let auth_service = AuthService::new(&state.db, &state.config.jwt);

    auth_service.revoke_token(&payload.refresh_token).await?;

    Ok(Json(serde_json::json!({ "message": "Logged out successfully" })))
}
