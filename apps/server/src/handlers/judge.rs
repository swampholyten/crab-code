use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    common::{
        response::{ApiResponse, ApiResult},
        state::AppState,
    },
    handlers::submission::{CreateSubmissionRequest, SubmissionResponse},
    models::judge::ExecutionLog,
};

#[derive(Debug, Serialize)]
pub struct ExecutionLogResponse {
    pub id: Uuid,
    pub submission_id: Uuid,
    pub language: String,
    pub execution_time: Option<i32>,
    pub memory_used: Option<i32>,
    pub exit_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<ExecutionLog> for ExecutionLogResponse {
    fn from(log: ExecutionLog) -> Self {
        Self {
            id: log.id,
            submission_id: log.submission_id,
            language: log.language,
            execution_time: log.execution_time,
            memory_used: log.memory_used,
            exit_code: log.exit_code,
            stdout: log.stdout,
            stderr: log.stderr,
            status: format!("{:?}", log.status).to_lowercase(),
            error_message: log.error_message,
            created_at: log.created_at,
        }
    }
}

pub async fn create_submission(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<CreateSubmissionRequest>,
) -> ApiResult<SubmissionResponse> {
    let create_request = crate::models::submission::CreateSubmissionRequest {
        user_id,
        problem_id: request.problem_id,
        language_id: request.language_id,
        code: request.code,
    };

    let submission = state
        .submission_service
        .create_submission(create_request)
        .await?;

    // Queue submission for judging
    state.judge_service.queue_submission(submission.id).await?;

    let response = ApiResponse::success_with_message(
        submission.into(),
        "Submission created successfully and queued for judging".to_string(),
    );

    Ok(Json(response))
}

// Add handler to get execution logs
pub async fn get_execution_logs(
    State(state): State<AppState>,
    Path(submission_id): Path<Uuid>,
) -> ApiResult<Vec<ExecutionLogResponse>> {
    let logs = state
        .judge_service
        .get_execution_logs(submission_id)
        .await?;

    let log_responses: Vec<ExecutionLogResponse> = logs.into_iter().map(|log| log.into()).collect();

    let response = ApiResponse::success(log_responses);
    Ok(Json(response))
}
