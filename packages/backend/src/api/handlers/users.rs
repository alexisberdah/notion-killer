use axum::{extract::State, Extension, Json};
use uuid::Uuid;

use crate::{
    api::handlers::auth::UserResponse,
    domain::entities::user::User,
    error::Result,
    AppState,
};

pub async fn get_me(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<UserResponse>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, name, avatar_url, email_verified, created_at, updated_at
        FROM users
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        user_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        name: user.name,
        avatar_url: user.avatar_url,
    }))
}
