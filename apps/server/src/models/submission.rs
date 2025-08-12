use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "submission_status", rename_all = "snake_case")]
pub enum SubmissionStatus {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    RuntimeError,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Submission {
    pub id: Uuid,
    pub user_id: Uuid,
    pub problem_id: Uuid,
    pub language_id: String,
    pub code: String,
    pub status: SubmissionStatus,
    pub execution_time: Option<i32>,
    pub memory_used: Option<i32>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubmissionRequest {
    pub user_id: Uuid,
    pub problem_id: Uuid,
    pub language_id: String,
    pub code: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubmissionFilter {
    pub status: Option<SubmissionStatus>,
    pub language_id: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeResult {
    pub status: SubmissionStatus,
    pub execution_time: Option<i32>,
    pub memory_used: Option<i32>,
    pub error_message: Option<String>,
    pub test_results: Vec<TestResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub input: String,
    pub expected_output: String,
    pub actual_output: Option<String>,
    pub passed: bool,
    pub execution_time: Option<i32>,
    pub memory_used: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub output: String,
    pub execution_time: i32,
    pub memory_used: i32,
    pub exit_code: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionLimits {
    pub time_limit: i32,      // in milliseconds
    pub memory_limit: i32,    // in KB
    pub code_size_limit: i32, // in bytes
}
