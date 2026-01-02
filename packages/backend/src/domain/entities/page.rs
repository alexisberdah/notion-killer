use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Page {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub title: String,
    pub icon: Option<String>,
    pub cover_url: Option<String>,
    pub is_database: bool,
    #[serde(skip_serializing)]
    pub crdt_state: Option<Vec<u8>>,
    pub crdt_vector_clock: serde_json::Value,
    pub created_by: Uuid,
    pub last_edited_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageWithChildren {
    #[serde(flatten)]
    pub page: Page,
    pub children: Vec<PageTreeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PageTreeItem {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub title: String,
    pub icon: Option<String>,
    pub is_database: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CrdtUpdate {
    pub id: i64,
    pub page_id: Uuid,
    pub client_id: i64,
    pub update_data: Vec<u8>,
    pub vector_clock: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
