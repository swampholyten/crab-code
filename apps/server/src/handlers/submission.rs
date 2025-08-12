use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::{
        response::{ApiResponse, ApiResult, PaginatedResponse},
        state::AppState,
    },
    models::submission::*,
};

// Request DTOs
#[derive(Debug, Deserialize)]
pub struct CreateSubmissionRequest {
    pub problem_id: Uuid,
    pub language_id: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct ListSubmissionsQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub status: Option<SubmissionStatus>,
    pub language_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubmissionResultRequest {
    pub status: SubmissionStatus,
    pub execution_time: Option<i32>,
    pub memory_used: Option<i32>,
    pub error_message: Option<String>,
    pub test_results: Option<Vec<TestResult>>,
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct SubmissionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub problem_id: Uuid,
    pub language_id: String,
    pub code: String,
    pub status: SubmissionStatus,
    pub execution_time: Option<i32>,
    pub memory_used: Option<i32>,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Submission> for SubmissionResponse {
    fn from(submission: Submission) -> Self {
        Self {
            id: submission.id,
            user_id: submission.user_id,
            problem_id: submission.problem_id,
            language_id: submission.language_id,
            code: submission.code,
            status: submission.status,
            execution_time: submission.execution_time,
            memory_used: submission.memory_used,
            error_message: submission.error_message,
            created_at: submission.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SubmissionSummaryResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub problem_id: Uuid,
    pub language_id: String,
    pub status: SubmissionStatus,
    pub execution_time: Option<i32>,
    pub memory_used: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Submission> for SubmissionSummaryResponse {
    fn from(submission: Submission) -> Self {
        Self {
            id: submission.id,
            user_id: submission.user_id,
            problem_id: submission.problem_id,
            language_id: submission.language_id,
            status: submission.status,
            execution_time: submission.execution_time,
            memory_used: submission.memory_used,
            created_at: submission.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SubmissionStatsResponse {
    pub total_submissions: i64,
    pub accepted_submissions: i64,
    pub wrong_answer_submissions: i64,
    pub time_limit_exceeded_submissions: i64,
    pub runtime_error_submissions: i64,
    pub acceptance_rate: f64,
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

pub async fn get_submission_by_id(
    State(state): State<AppState>,
    Path(submission_id): Path<Uuid>,
) -> ApiResult<SubmissionResponse> {
    let submission = state
        .submission_service
        .get_submission_by_id(submission_id)
        .await?
        .ok_or_else(|| crate::errors::Error::NotFound("Submission not found".to_string()))?;

    let response = ApiResponse::success(submission.into());
    Ok(Json(response))
}

pub async fn get_user_submissions(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<ListSubmissionsQuery>,
) -> ApiResult<PaginatedResponse<SubmissionSummaryResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let filter = SubmissionFilter {
        status: query.status.clone(),
        language_id: query.language_id.clone(),
        limit: Some(per_page),
        offset: Some(offset),
    };

    let submissions = state
        .submission_service
        .get_user_submissions(user_id, filter)
        .await?;

    let total = submissions.len() as i64;

    let responses = submissions
        .into_iter()
        .map(SubmissionSummaryResponse::from)
        .collect();

    Ok(Json(ApiResponse::success(PaginatedResponse::new(
        responses, total, page, per_page,
    ))))
}

pub async fn get_problem_submissions(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
    Query(query): Query<ListSubmissionsQuery>,
) -> ApiResult<PaginatedResponse<SubmissionSummaryResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let filter = SubmissionFilter {
        status: query.status.clone(), // clone once
        language_id: query.language_id.clone(),
        limit: Some(per_page),
        offset: Some(offset),
    };

    let submissions = state
        .submission_service
        .get_problem_submissions(problem_id, filter)
        .await?;

    let total = submissions.len() as i64;

    let responses = submissions
        .into_iter()
        .map(SubmissionSummaryResponse::from)
        .collect();

    Ok(Json(ApiResponse::success(PaginatedResponse::new(
        responses, total, page, per_page,
    ))))
}

pub async fn update_submission_result(
    State(state): State<AppState>,
    Path(submission_id): Path<Uuid>,
    Json(request): Json<UpdateSubmissionResultRequest>,
) -> ApiResult<()> {
    let judge_result = JudgeResult {
        status: request.status,
        execution_time: request.execution_time,
        memory_used: request.memory_used,
        error_message: request.error_message,
        test_results: request.test_results.unwrap_or_default(),
    };

    state
        .submission_service
        .update_submission_result(submission_id, judge_result)
        .await?;

    let response =
        ApiResponse::success_message("Submission result updated successfully".to_string());
    Ok(Json(response))
}

pub async fn get_user_submission_stats(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<SubmissionStatsResponse> {
    let filter = SubmissionFilter {
        status: None,
        language_id: None,
        limit: None,
        offset: None,
    };

    let submissions = state
        .submission_service
        .get_user_submissions(user_id, filter)
        .await?;

    let total = submissions.len() as i64;
    let accepted = submissions
        .iter()
        .filter(|s| s.status == SubmissionStatus::Accepted)
        .count() as i64;
    let wrong_answer = submissions
        .iter()
        .filter(|s| s.status == SubmissionStatus::WrongAnswer)
        .count() as i64;
    let time_limit = submissions
        .iter()
        .filter(|s| s.status == SubmissionStatus::TimeLimitExceeded)
        .count() as i64;
    let runtime_error = submissions
        .iter()
        .filter(|s| s.status == SubmissionStatus::RuntimeError)
        .count() as i64;

    let acceptance_rate = if total > 0 {
        (accepted as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let stats = SubmissionStatsResponse {
        total_submissions: total,
        accepted_submissions: accepted,
        wrong_answer_submissions: wrong_answer,
        time_limit_exceeded_submissions: time_limit,
        runtime_error_submissions: runtime_error,
        acceptance_rate,
    };

    let response = ApiResponse::success(stats);
    Ok(Json(response))
}

pub async fn get_problem_submission_stats(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
) -> ApiResult<SubmissionStatsResponse> {
    let filter = SubmissionFilter {
        status: None,
        language_id: None,
        limit: None,
        offset: None,
    };

    let submissions = state
        .submission_service
        .get_problem_submissions(problem_id, filter)
        .await?;

    let total = submissions.len() as i64;
    let accepted = submissions
        .iter()
        .filter(|s| s.status == SubmissionStatus::Accepted)
        .count() as i64;
    let wrong_answer = submissions
        .iter()
        .filter(|s| s.status == SubmissionStatus::WrongAnswer)
        .count() as i64;
    let time_limit = submissions
        .iter()
        .filter(|s| s.status == SubmissionStatus::TimeLimitExceeded)
        .count() as i64;
    let runtime_error = submissions
        .iter()
        .filter(|s| s.status == SubmissionStatus::RuntimeError)
        .count() as i64;

    let acceptance_rate = if total > 0 {
        (accepted as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let stats = SubmissionStatsResponse {
        total_submissions: total,
        accepted_submissions: accepted,
        wrong_answer_submissions: wrong_answer,
        time_limit_exceeded_submissions: time_limit,
        runtime_error_submissions: runtime_error,
        acceptance_rate,
    };

    let response = ApiResponse::success(stats);
    Ok(Json(response))
}

// Utility handler to get submission code (for authorized users only)
pub async fn get_submission_code(
    State(state): State<AppState>,
    Path(submission_id): Path<Uuid>,
    // TODO: Add authentication extraction here
    // Extract(user): Extract<AuthenticatedUser>,
) -> ApiResult<serde_json::Value> {
    let submission = state
        .submission_service
        .get_submission_by_id(submission_id)
        .await?
        .ok_or_else(|| crate::errors::Error::NotFound("Submission not found".to_string()))?;

    // TODO: Add authorization check - user can only see their own submissions
    // if user.id != submission.user_id && !user.is_admin() {
    //     return Err(ServiceError::ForbiddenError("Access denied".to_string()).into());
    // }

    let code_response = serde_json::json!({
        "submission_id": submission.id,
        "code": submission.code,
        "language_id": submission.language_id
    });

    let response = ApiResponse::success(code_response);
    Ok(Json(response))
}
