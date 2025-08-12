use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "difficulty_level", rename_all = "lowercase")]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Problem {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub difficulty: DifficultyLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProblemRequest {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub difficulty: DifficultyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProblemRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<DifficultyLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemWithTags {
    #[serde(flatten)]
    pub problem: Problem,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProblemFilter {
    pub difficulty: Option<DifficultyLevel>,
    pub tags: Option<Vec<String>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub search: Option<String>,
}
