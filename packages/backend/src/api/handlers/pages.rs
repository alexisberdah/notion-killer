use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::entities::page::{Page, PageTreeItem},
    error::{AppError, Result},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListPagesQuery {
    pub workspace_id: Uuid,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePageRequest {
    pub workspace_id: Uuid,
    pub parent_id: Option<Uuid>,
    #[validate(length(max = 500, message = "Title must be at most 500 characters"))]
    pub title: Option<String>,
    pub icon: Option<String>,
    pub cover_url: Option<String>,
    pub is_database: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePageRequest {
    #[validate(length(max = 500, message = "Title must be at most 500 characters"))]
    pub title: Option<String>,
    pub icon: Option<String>,
    pub cover_url: Option<String>,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct PageResponse {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub title: String,
    pub icon: Option<String>,
    pub cover_url: Option<String>,
    pub is_database: bool,
    pub created_by: Uuid,
    pub last_edited_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Page> for PageResponse {
    fn from(p: Page) -> Self {
        Self {
            id: p.id,
            workspace_id: p.workspace_id,
            parent_id: p.parent_id,
            title: p.title,
            icon: p.icon,
            cover_url: p.cover_url,
            is_database: p.is_database,
            created_by: p.created_by,
            last_edited_by: p.last_edited_by,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PageTreeResponse {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub title: String,
    pub icon: Option<String>,
    pub is_database: bool,
    pub children: Vec<PageTreeResponse>,
}

/// List pages in a workspace (with optional parent filter)
pub async fn list_pages(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Query(query): Query<ListPagesQuery>,
) -> Result<Json<Vec<PageResponse>>> {
    // Verify user has access to workspace
    let member: Option<(String,)> = sqlx::query_as(
        r#"SELECT role FROM workspace_members WHERE workspace_id = $1 AND user_id = $2"#
    )
    .bind(query.workspace_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    if member.is_none() {
        return Err(AppError::Forbidden);
    }

    let pages: Vec<Page> = match query.parent_id {
        Some(parent_id) => {
            sqlx::query_as(
                r#"
                SELECT id, workspace_id, parent_id, title, icon, cover_url, is_database,
                       crdt_state, crdt_vector_clock, created_by, last_edited_by, created_at, updated_at
                FROM pages
                WHERE workspace_id = $1 AND parent_id = $2 AND deleted_at IS NULL
                ORDER BY created_at ASC
                "#
            )
            .bind(query.workspace_id)
            .bind(parent_id)
            .fetch_all(&state.db)
            .await?
        }
        None => {
            sqlx::query_as(
                r#"
                SELECT id, workspace_id, parent_id, title, icon, cover_url, is_database,
                       crdt_state, crdt_vector_clock, created_by, last_edited_by, created_at, updated_at
                FROM pages
                WHERE workspace_id = $1 AND parent_id IS NULL AND deleted_at IS NULL
                ORDER BY created_at ASC
                "#
            )
            .bind(query.workspace_id)
            .fetch_all(&state.db)
            .await?
        }
    };

    Ok(Json(pages.into_iter().map(Into::into).collect()))
}

/// Get page tree for sidebar
pub async fn get_page_tree(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<Vec<PageTreeResponse>>> {
    // Verify user has access to workspace
    let member: Option<(String,)> = sqlx::query_as(
        r#"SELECT role FROM workspace_members WHERE workspace_id = $1 AND user_id = $2"#
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    if member.is_none() {
        return Err(AppError::Forbidden);
    }

    // Get all pages for the workspace
    let pages: Vec<PageTreeItem> = sqlx::query_as(
        r#"
        SELECT id, parent_id, title, icon, is_database
        FROM pages
        WHERE workspace_id = $1 AND deleted_at IS NULL
        ORDER BY created_at ASC
        "#
    )
    .bind(workspace_id)
    .fetch_all(&state.db)
    .await?;

    // Build tree structure
    let tree = build_page_tree(pages, None);

    Ok(Json(tree))
}

fn build_page_tree(pages: Vec<PageTreeItem>, parent_id: Option<Uuid>) -> Vec<PageTreeResponse> {
    pages
        .iter()
        .filter(|p| p.parent_id == parent_id)
        .map(|p| PageTreeResponse {
            id: p.id,
            parent_id: p.parent_id,
            title: p.title.clone(),
            icon: p.icon.clone(),
            is_database: p.is_database,
            children: build_page_tree(pages.clone(), Some(p.id)),
        })
        .collect()
}

/// Create a new page
pub async fn create_page(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreatePageRequest>,
) -> Result<Json<PageResponse>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    // Verify user has access to workspace
    let member: Option<(String,)> = sqlx::query_as(
        r#"SELECT role FROM workspace_members WHERE workspace_id = $1 AND user_id = $2"#
    )
    .bind(payload.workspace_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    if member.is_none() {
        return Err(AppError::Forbidden);
    }

    // If parent_id is set, verify it exists and belongs to the same workspace
    if let Some(parent_id) = payload.parent_id {
        let parent: Option<(Uuid,)> = sqlx::query_as(
            r#"SELECT workspace_id FROM pages WHERE id = $1 AND deleted_at IS NULL"#
        )
        .bind(parent_id)
        .fetch_optional(&state.db)
        .await?;

        match parent {
            Some((parent_workspace_id,)) if parent_workspace_id != payload.workspace_id => {
                return Err(AppError::Validation("Parent page must be in the same workspace".to_string()));
            }
            None => {
                return Err(AppError::NotFound);
            }
            _ => {}
        }
    }

    let page_id = Uuid::new_v4();
    let title = payload.title.unwrap_or_else(|| "Untitled".to_string());

    let page: Page = sqlx::query_as(
        r#"
        INSERT INTO pages (id, workspace_id, parent_id, title, icon, cover_url, is_database, created_by, last_edited_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
        RETURNING id, workspace_id, parent_id, title, icon, cover_url, is_database,
                  crdt_state, crdt_vector_clock, created_by, last_edited_by, created_at, updated_at
        "#
    )
    .bind(page_id)
    .bind(payload.workspace_id)
    .bind(payload.parent_id)
    .bind(&title)
    .bind(&payload.icon)
    .bind(&payload.cover_url)
    .bind(payload.is_database.unwrap_or(false))
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(page.into()))
}

/// Get a single page
pub async fn get_page(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(page_id): Path<Uuid>,
) -> Result<Json<PageResponse>> {
    let page: Page = sqlx::query_as(
        r#"
        SELECT p.id, p.workspace_id, p.parent_id, p.title, p.icon, p.cover_url, p.is_database,
               p.crdt_state, p.crdt_vector_clock, p.created_by, p.last_edited_by, p.created_at, p.updated_at
        FROM pages p
        INNER JOIN workspace_members wm ON p.workspace_id = wm.workspace_id
        WHERE p.id = $1 AND wm.user_id = $2 AND p.deleted_at IS NULL
        "#
    )
    .bind(page_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(page.into()))
}

/// Update a page
pub async fn update_page(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(page_id): Path<Uuid>,
    Json(payload): Json<UpdatePageRequest>,
) -> Result<Json<PageResponse>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    // Verify user has access to the page
    let existing: Option<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT p.workspace_id
        FROM pages p
        INNER JOIN workspace_members wm ON p.workspace_id = wm.workspace_id
        WHERE p.id = $1 AND wm.user_id = $2 AND p.deleted_at IS NULL
        "#
    )
    .bind(page_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    let (workspace_id,) = existing.ok_or(AppError::NotFound)?;

    // If parent_id is being changed, verify the new parent
    if let Some(new_parent_id) = payload.parent_id {
        // Prevent setting page as its own parent
        if new_parent_id == page_id {
            return Err(AppError::Validation("Page cannot be its own parent".to_string()));
        }

        let parent: Option<(Uuid,)> = sqlx::query_as(
            r#"SELECT workspace_id FROM pages WHERE id = $1 AND deleted_at IS NULL"#
        )
        .bind(new_parent_id)
        .fetch_optional(&state.db)
        .await?;

        match parent {
            Some((parent_workspace_id,)) if parent_workspace_id != workspace_id => {
                return Err(AppError::Validation("Parent page must be in the same workspace".to_string()));
            }
            None => {
                return Err(AppError::NotFound);
            }
            _ => {}
        }

        // TODO: Check for circular references
    }

    let page: Page = sqlx::query_as(
        r#"
        UPDATE pages
        SET
            title = COALESCE($1, title),
            icon = COALESCE($2, icon),
            cover_url = COALESCE($3, cover_url),
            parent_id = COALESCE($4, parent_id),
            last_edited_by = $5,
            updated_at = NOW()
        WHERE id = $6 AND deleted_at IS NULL
        RETURNING id, workspace_id, parent_id, title, icon, cover_url, is_database,
                  crdt_state, crdt_vector_clock, created_by, last_edited_by, created_at, updated_at
        "#
    )
    .bind(&payload.title)
    .bind(&payload.icon)
    .bind(&payload.cover_url)
    .bind(payload.parent_id)
    .bind(user_id)
    .bind(page_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(page.into()))
}

/// Delete a page (soft delete)
pub async fn delete_page(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(page_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    // Verify user has access to the page
    let existing: Option<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT p.workspace_id
        FROM pages p
        INNER JOIN workspace_members wm ON p.workspace_id = wm.workspace_id
        WHERE p.id = $1 AND wm.user_id = $2 AND p.deleted_at IS NULL
        "#
    )
    .bind(page_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    if existing.is_none() {
        return Err(AppError::NotFound);
    }

    // Soft delete the page and all its children
    sqlx::query(
        r#"
        WITH RECURSIVE page_tree AS (
            SELECT id FROM pages WHERE id = $1
            UNION ALL
            SELECT p.id FROM pages p
            INNER JOIN page_tree pt ON p.parent_id = pt.id
        )
        UPDATE pages SET deleted_at = NOW()
        WHERE id IN (SELECT id FROM page_tree)
        "#
    )
    .bind(page_id)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({ "message": "Page deleted" })))
}

/// Duplicate a page
pub async fn duplicate_page(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(page_id): Path<Uuid>,
) -> Result<Json<PageResponse>> {
    // Get original page
    let original: Page = sqlx::query_as(
        r#"
        SELECT p.id, p.workspace_id, p.parent_id, p.title, p.icon, p.cover_url, p.is_database,
               p.crdt_state, p.crdt_vector_clock, p.created_by, p.last_edited_by, p.created_at, p.updated_at
        FROM pages p
        INNER JOIN workspace_members wm ON p.workspace_id = wm.workspace_id
        WHERE p.id = $1 AND wm.user_id = $2 AND p.deleted_at IS NULL
        "#
    )
    .bind(page_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    let new_page_id = Uuid::new_v4();
    let new_title = format!("{} (Copy)", original.title);

    let page: Page = sqlx::query_as(
        r#"
        INSERT INTO pages (id, workspace_id, parent_id, title, icon, cover_url, is_database, crdt_state, created_by, last_edited_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)
        RETURNING id, workspace_id, parent_id, title, icon, cover_url, is_database,
                  crdt_state, crdt_vector_clock, created_by, last_edited_by, created_at, updated_at
        "#
    )
    .bind(new_page_id)
    .bind(original.workspace_id)
    .bind(original.parent_id)
    .bind(&new_title)
    .bind(&original.icon)
    .bind(&original.cover_url)
    .bind(original.is_database)
    .bind(&original.crdt_state)
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(page.into()))
}

/// Move a page to a new parent
pub async fn move_page(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(page_id): Path<Uuid>,
    Json(payload): Json<MovePageRequest>,
) -> Result<Json<PageResponse>> {
    // Get current page
    let existing: Page = sqlx::query_as(
        r#"
        SELECT p.id, p.workspace_id, p.parent_id, p.title, p.icon, p.cover_url, p.is_database,
               p.crdt_state, p.crdt_vector_clock, p.created_by, p.last_edited_by, p.created_at, p.updated_at
        FROM pages p
        INNER JOIN workspace_members wm ON p.workspace_id = wm.workspace_id
        WHERE p.id = $1 AND wm.user_id = $2 AND p.deleted_at IS NULL
        "#
    )
    .bind(page_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    // Validate new parent
    if let Some(new_parent_id) = payload.new_parent_id {
        if new_parent_id == page_id {
            return Err(AppError::Validation("Page cannot be its own parent".to_string()));
        }

        // Check new parent exists and is in same workspace
        let parent: Option<(Uuid,)> = sqlx::query_as(
            r#"SELECT workspace_id FROM pages WHERE id = $1 AND deleted_at IS NULL"#
        )
        .bind(new_parent_id)
        .fetch_optional(&state.db)
        .await?;

        match parent {
            Some((parent_workspace_id,)) if parent_workspace_id != existing.workspace_id => {
                return Err(AppError::Validation("Parent page must be in the same workspace".to_string()));
            }
            None => {
                return Err(AppError::NotFound);
            }
            _ => {}
        }

        // Check for circular reference
        let is_descendant: Option<(i32,)> = sqlx::query_as(
            r#"
            WITH RECURSIVE descendants AS (
                SELECT id, 1 as depth FROM pages WHERE parent_id = $1
                UNION ALL
                SELECT p.id, d.depth + 1
                FROM pages p
                INNER JOIN descendants d ON p.parent_id = d.id
                WHERE d.depth < 100
            )
            SELECT 1 FROM descendants WHERE id = $2 LIMIT 1
            "#
        )
        .bind(page_id)
        .bind(new_parent_id)
        .fetch_optional(&state.db)
        .await?;

        if is_descendant.is_some() {
            return Err(AppError::Validation("Cannot move page to one of its descendants".to_string()));
        }
    }

    let page: Page = sqlx::query_as(
        r#"
        UPDATE pages
        SET parent_id = $1, last_edited_by = $2, updated_at = NOW()
        WHERE id = $3 AND deleted_at IS NULL
        RETURNING id, workspace_id, parent_id, title, icon, cover_url, is_database,
                  crdt_state, crdt_vector_clock, created_by, last_edited_by, created_at, updated_at
        "#
    )
    .bind(payload.new_parent_id)
    .bind(user_id)
    .bind(page_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(page.into()))
}

#[derive(Debug, Deserialize)]
pub struct MovePageRequest {
    pub new_parent_id: Option<Uuid>,
}

/// Get breadcrumbs for a page
pub async fn get_breadcrumbs(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(page_id): Path<Uuid>,
) -> Result<Json<Vec<BreadcrumbItem>>> {
    // Verify access
    let access: Option<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT p.workspace_id
        FROM pages p
        INNER JOIN workspace_members wm ON p.workspace_id = wm.workspace_id
        WHERE p.id = $1 AND wm.user_id = $2 AND p.deleted_at IS NULL
        "#
    )
    .bind(page_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    if access.is_none() {
        return Err(AppError::NotFound);
    }

    let breadcrumbs: Vec<BreadcrumbItem> = sqlx::query_as(
        r#"
        WITH RECURSIVE ancestors AS (
            SELECT id, parent_id, title, icon, 0 as depth
            FROM pages WHERE id = $1
            UNION ALL
            SELECT p.id, p.parent_id, p.title, p.icon, a.depth + 1
            FROM pages p
            INNER JOIN ancestors a ON p.id = a.parent_id
            WHERE a.depth < 50
        )
        SELECT id, title, icon
        FROM ancestors
        ORDER BY depth DESC
        "#
    )
    .bind(page_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(breadcrumbs))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct BreadcrumbItem {
    pub id: Uuid,
    pub title: String,
    pub icon: Option<String>,
}
