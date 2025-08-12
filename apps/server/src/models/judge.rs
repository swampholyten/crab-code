use crate::models::submission::SubmissionStatus;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExecutionLog {
    pub id: Uuid,
    pub submission_id: Uuid,
    pub language: String,
    pub execution_time: Option<i32>,
    pub memory_used: Option<i32>,
    pub exit_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub status: SubmissionStatus,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExecutionLogRequest {
    pub submission_id: Uuid,
    pub language: String,
    pub execution_time: Option<i32>,
    pub memory_used: Option<i32>,
    pub exit_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub status: SubmissionStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub name: String,
    pub display_name: String,
    pub file_extension: String,
    pub compile_command: Option<String>,
    pub execute_command: String,
    pub time_limit: i32,   // 5 seconds
    pub memory_limit: i32, // 256 MB
}

impl From<crate::models::language::Language> for LanguageConfig {
    fn from(lang: crate::models::language::Language) -> Self {
        let (compile_command, execute_command, time_limit, memory_limit) = match lang.name.as_str()
        {
            "python" => (None, "python3 solution.py".to_string(), 5000, 256 * 1024),

            "c" => (
                Some("gcc -o solution solution.c".to_string()),
                "./solution".to_string(),
                3000,
                256 * 1024,
            ),

            "rust" => (
                Some("rustc solution.rs -o solution".to_string()),
                "./solution".to_string(),
                5000,
                256 * 1024,
            ),

            "go" => (
                Some("go build -o solution solution.go".to_string()),
                "./solution".to_string(),
                5000,
                256 * 1024,
            ),

            "typescript" => (
                Some("tsc solution.ts".to_string()),
                "node solution.js".to_string(),
                5000,
                256 * 1024,
            ),

            _ => (
                None,
                format!("echo 'Unsupported language: {}'", lang.name),
                5000,
                256 * 1024,
            ),
        };

        Self {
            name: lang.name,
            display_name: lang.display_name,
            file_extension: lang.file_extension,
            compile_command,
            execute_command,
            time_limit,
            memory_limit,
        }
    }
}

