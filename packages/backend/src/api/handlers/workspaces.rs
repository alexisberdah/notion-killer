use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::entities::workspace::Workspace,
    error::{AppError, Result},
    AppState,
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateWorkspaceRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: String,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateWorkspaceRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WorkspaceResponse {
    pub id: Uuid,
    pub name: String,
    pub icon: Option<String>,
    pub owner_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Workspace> for WorkspaceResponse {
    fn from(w: Workspace) -> Self {
        Self {
            id: w.id,
            name: w.name,
            icon: w.icon,
            owner_id: w.owner_id,
            created_at: w.created_at,
        }
    }
}

pub async fn list_workspaces(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Vec<WorkspaceResponse>>> {
    let workspaces: Vec<Workspace> = sqlx::query_as(
        r#"
        SELECT w.id, w.name, w.icon, w.owner_id, w.settings, w.created_at, w.updated_at
        FROM workspaces w
        INNER JOIN workspace_members wm ON w.id = wm.workspace_id
        WHERE wm.user_id = $1 AND w.deleted_at IS NULL
        ORDER BY w.created_at DESC
        "#
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(workspaces.into_iter().map(Into::into).collect()))
}

pub async fn create_workspace(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreateWorkspaceRequest>,
) -> Result<Json<WorkspaceResponse>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let workspace_id = Uuid::new_v4();

    // Create workspace and add owner as member in a transaction
    let mut tx = state.db.begin().await?;

    let workspace: Workspace = sqlx::query_as(
        r#"
        INSERT INTO workspaces (id, name, icon, owner_id, settings)
        VALUES ($1, $2, $3, $4, '{}')
        RETURNING id, name, icon, owner_id, settings, created_at, updated_at
        "#
    )
    .bind(workspace_id)
    .bind(&payload.name)
    .bind(&payload.icon)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    // Add owner as member with 'owner' role
    sqlx::query(
        r#"
        INSERT INTO workspace_members (workspace_id, user_id, role, invited_by)
        VALUES ($1, $2, 'owner', $2)
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(workspace.into()))
}

pub async fn get_workspace(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<WorkspaceResponse>> {
    let workspace: Workspace = sqlx::query_as(
        r#"
        SELECT w.id, w.name, w.icon, w.owner_id, w.settings, w.created_at, w.updated_at
        FROM workspaces w
        INNER JOIN workspace_members wm ON w.id = wm.workspace_id
        WHERE w.id = $1 AND wm.user_id = $2 AND w.deleted_at IS NULL
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(workspace.into()))
}

pub async fn update_workspace(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(workspace_id): Path<Uuid>,
    Json(payload): Json<UpdateWorkspaceRequest>,
) -> Result<Json<WorkspaceResponse>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    // Check if user has permission to update
    let member: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT role FROM workspace_members
        WHERE workspace_id = $1 AND user_id = $2
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    let (role,) = member.ok_or(AppError::NotFound)?;

    if role != "owner" && role != "admin" {
        return Err(AppError::Forbidden);
    }

    let workspace: Workspace = sqlx::query_as(
        r#"
        UPDATE workspaces
        SET
            name = COALESCE($1, name),
            icon = COALESCE($2, icon),
            updated_at = NOW()
        WHERE id = $3 AND deleted_at IS NULL
        RETURNING id, name, icon, owner_id, settings, created_at, updated_at
        "#
    )
    .bind(&payload.name)
    .bind(&payload.icon)
    .bind(workspace_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(workspace.into()))
}

pub async fn delete_workspace(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    // Only owner can delete
    let workspace: Option<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT owner_id FROM workspaces
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(workspace_id)
    .fetch_optional(&state.db)
    .await?;

    let (owner_id,) = workspace.ok_or(AppError::NotFound)?;

    if owner_id != user_id {
        return Err(AppError::Forbidden);
    }

    // Soft delete
    sqlx::query(
        r#"
        UPDATE workspaces
        SET deleted_at = NOW()
        WHERE id = $1
        "#
    )
    .bind(workspace_id)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({ "message": "Workspace deleted" })))
}
