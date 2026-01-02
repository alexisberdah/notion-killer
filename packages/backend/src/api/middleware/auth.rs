use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    domain::services::auth_service::AuthService,
    error::{AppError, Result},
    AppState,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let auth_service = AuthService::new(&state.db, &state.config.jwt);
    let user_id = auth_service.verify_access_token(token)?;

    // Add user_id to request extensions
    request.extensions_mut().insert(user_id);

    Ok(next.run(request).await)
}

// Helper to extract user_id from request
pub fn get_user_id(extensions: &axum::http::Extensions) -> Result<Uuid> {
    extensions
        .get::<Uuid>()
        .copied()
        .ok_or(AppError::Unauthorized)
}
