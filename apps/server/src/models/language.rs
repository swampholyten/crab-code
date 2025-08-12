use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub display_name: String,
    pub file_extension: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLanguageRequest {
    pub name: String,
    pub display_name: String,
    pub file_extension: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLanguageRequest {
    pub display_name: Option<String>,
    pub file_extension: Option<String>,
}
