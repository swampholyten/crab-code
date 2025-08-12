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
    models::problem::*,
};

// Request DTOs
#[derive(Debug, Deserialize)]
pub struct CreateProblemRequest {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub difficulty: DifficultyLevel,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProblemRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<DifficultyLevel>,
}

#[derive(Debug, Deserialize)]
pub struct ListProblemsQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub difficulty: Option<DifficultyLevel>,
    pub tags: Option<Vec<String>>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchProblemsQuery {
    pub q: String,
    pub limit: Option<i32>,
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct ProblemResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub difficulty: DifficultyLevel,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Problem> for ProblemResponse {
    fn from(problem: Problem) -> Self {
        Self {
            id: problem.id,
            title: problem.title,
            slug: problem.slug,
            description: problem.description,
            difficulty: problem.difficulty,
            created_at: problem.created_at,
            updated_at: problem.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProblemSummaryResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub difficulty: DifficultyLevel,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Problem> for ProblemSummaryResponse {
    fn from(problem: Problem) -> Self {
        Self {
            id: problem.id,
            title: problem.title,
            slug: problem.slug,
            difficulty: problem.difficulty,
            created_at: problem.created_at,
        }
    }
}

pub async fn create_problem(
    State(state): State<AppState>,
    Json(request): Json<CreateProblemRequest>,
) -> ApiResult<ProblemResponse> {
    let create_request = crate::models::problem::CreateProblemRequest {
        title: request.title,
        slug: request.slug,
        description: request.description,
        difficulty: request.difficulty,
    };

    let problem = state.problem_service.create_problem(create_request).await?;
    let response = ApiResponse::success_with_message(
        problem.into(),
        "Problem created successfully".to_string(),
    );

    Ok(Json(response))
}

pub async fn get_problem_by_id(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
) -> ApiResult<ProblemResponse> {
    let problem = state
        .problem_service
        .get_problem_by_id(problem_id)
        .await?
        .ok_or_else(|| crate::errors::Error::NotFound("Problem not found".to_string()))?;

    let response = ApiResponse::success(problem.into());
    Ok(Json(response))
}

pub async fn get_problem_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> ApiResult<ProblemResponse> {
    let problem = state
        .problem_service
        .get_problem_by_slug(&slug)
        .await?
        .ok_or_else(|| crate::errors::Error::NotFound("Problem not found".to_string()))?;

    let response = ApiResponse::success(problem.into());
    Ok(Json(response))
}

pub async fn update_problem(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
    Json(request): Json<UpdateProblemRequest>,
) -> ApiResult<ProblemResponse> {
    let update_request = crate::models::problem::UpdateProblemRequest {
        title: request.title,
        slug: request.slug,
        description: request.description,
        difficulty: request.difficulty,
    };

    let problem = state
        .problem_service
        .update_problem(problem_id, update_request)
        .await?;
    let response = ApiResponse::success_with_message(
        problem.into(),
        "Problem updated successfully".to_string(),
    );

    Ok(Json(response))
}

pub async fn delete_problem(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
) -> crate::errors::Result<Json<ApiResponse<()>>> {
    state.problem_service.delete_problem(problem_id).await?;
    let response = ApiResponse::success_message("Problem deleted successfully".to_string());
    Ok(Json(response))
}

pub async fn list_problems(
    State(state): State<AppState>,
    Query(query): Query<ListProblemsQuery>,
) -> ApiResult<PaginatedResponse<ProblemSummaryResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * per_page;

    let filter = crate::models::problem::ProblemFilter {
        difficulty: query.difficulty,
        tags: query.tags,
        search: query.search,
        limit: Some(per_page),
        offset: Some(offset),
    };

    // Get total count
    let total = state.problem_service.count_problems().await?;

    // Get problems
    let problems = state.problem_service.list_problems(filter).await?;

    let problem_responses: Vec<ProblemSummaryResponse> =
        problems.into_iter().map(|p| p.into()).collect();
    let paginated = PaginatedResponse::new(problem_responses, total, page, per_page);

    let response = ApiResponse::success(paginated);
    Ok(Json(response))
}

pub async fn search_problems(
    State(state): State<AppState>,
    Query(query): Query<SearchProblemsQuery>,
) -> ApiResult<Vec<ProblemSummaryResponse>> {
    let problems = state.problem_service.search_problems(&query.q).await?;

    let mut problem_responses: Vec<ProblemSummaryResponse> =
        problems.into_iter().map(|p| p.into()).collect();

    // Apply limit if specified
    if let Some(limit) = query.limit {
        problem_responses.truncate(limit as usize);
    }

    let response = ApiResponse::success(problem_responses);
    Ok(Json(response))
}

pub async fn get_problems_by_difficulty(
    State(state): State<AppState>,
    Path(difficulty): Path<DifficultyLevel>,
) -> ApiResult<Vec<ProblemSummaryResponse>> {
    let problems = state
        .problem_service
        .get_problems_by_difficulty(difficulty)
        .await?;

    let problem_responses: Vec<ProblemSummaryResponse> =
        problems.into_iter().map(|p| p.into()).collect();
    let response = ApiResponse::success(problem_responses);
    Ok(Json(response))
}
