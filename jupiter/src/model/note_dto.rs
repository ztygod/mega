use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteDto {
    pub id: String,
    pub title: String,
    pub description_html: String,
    pub description_state: Option<String>,
    pub description_schema_version: i32,

    pub created_at: String,
    pub last_activity_at: String,
    pub content_updated_at: String,

    // pub project_id: Option<String>,
    // pub member_id: String,
    pub comments_count: i64,
    pub resolved_comments_count: i64,
}

/// Query parameters for getting notes (internal)  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNotesParams {
    pub cursor: Option<String>,
    pub limit: u64,
    pub query: Option<String>,
    pub order_by: Option<NoteOrderBy>,
    pub order_direction: Option<NoteOrderDirection>,
    pub org_slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NoteOrderBy {
    CreatedAt,
    LastActivityAt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NoteOrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNotesRes {
    pub next_cursor: Option<String>,
    pub prev_cursor: Option<String>,
    pub data: Vec<NoteDto>,
}
