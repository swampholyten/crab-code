use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TestCase {
    pub id: Uuid,
    pub problem_id: Uuid,
    pub input_data: String,
    pub expected_output: String,
    pub is_sample: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTestCaseRequest {
    pub problem_id: Uuid,
    pub input_data: String,
    pub expected_output: String,
    pub is_sample: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTestCaseRequest {
    pub input_data: Option<String>,
    pub expected_output: Option<String>,
    pub is_sample: Option<bool>,
}
