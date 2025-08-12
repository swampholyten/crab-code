use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::{
        response::{ApiResponse, ApiResult},
        state::AppState,
    },
    models::{problem::Problem, tag::*},
};

// Request DTOs
#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PopularTagsQuery {
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct BulkTagRequest {
    pub tag_names: Vec<String>,
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        Self {
            name: tag.name,
            description: tag.description,
            created_at: tag.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TagWithCountResponse {
    pub name: String,
    pub description: Option<String>,
    pub problem_count: i64,
}

impl From<TagWithCount> for TagWithCountResponse {
    fn from(tag: TagWithCount) -> Self {
        Self {
            name: tag.name,
            description: tag.description,
            problem_count: tag.problem_count,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProblemSummaryResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub difficulty: crate::models::problem::DifficultyLevel,
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

// Handler functions
pub async fn create_tag(
    State(state): State<AppState>,
    Json(request): Json<CreateTagRequest>,
) -> ApiResult<TagResponse> {
    let create_request = crate::models::tag::CreateTagRequest {
        name: request.name,
        description: request.description,
    };

    let tag = state.tag_service.create_tag(create_request).await?;
    let response =
        ApiResponse::success_with_message(tag.into(), "Tag created successfully".to_string());

    Ok(Json(response))
}

pub async fn get_all_tags(State(state): State<AppState>) -> ApiResult<Vec<TagResponse>> {
    let tags = state.tag_service.get_all_tags().await?;

    let tag_responses: Vec<TagResponse> = tags.into_iter().map(|tag| tag.into()).collect();

    let response = ApiResponse::success(tag_responses);
    Ok(Json(response))
}

pub async fn get_tag_by_name(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ApiResult<TagResponse> {
    let tag = state
        .tag_service
        .get_tag_by_name(&name)
        .await?
        .ok_or_else(|| crate::errors::Error::NotFound(format!("Tag '{}' not found", name)))?;

    let response = ApiResponse::success(tag.into());
    Ok(Json(response))
}

pub async fn delete_tag(State(state): State<AppState>, Path(name): Path<String>) -> ApiResult<()> {
    state.tag_service.delete_tag(&name).await?;
    let response = ApiResponse::success_message(format!("Tag '{}' deleted successfully", name));
    Ok(Json(response))
}

pub async fn get_problems_by_tag(
    State(state): State<AppState>,
    Path(tag_name): Path<String>,
) -> ApiResult<Vec<ProblemSummaryResponse>> {
    let problems = state.tag_service.get_problems_by_tag(&tag_name).await?;

    let problem_responses: Vec<ProblemSummaryResponse> =
        problems.into_iter().map(|problem| problem.into()).collect();

    let response = ApiResponse::success(problem_responses);
    Ok(Json(response))
}

pub async fn add_tag_to_problem(
    State(state): State<AppState>,
    Path((problem_id, tag_name)): Path<(Uuid, String)>,
) -> ApiResult<()> {
    state
        .tag_service
        .add_tag_to_problem(problem_id, &tag_name)
        .await?;
    let response =
        ApiResponse::success_message(format!("Tag '{}' added to problem successfully", tag_name));
    Ok(Json(response))
}

pub async fn remove_tag_from_problem(
    State(state): State<AppState>,
    Path((problem_id, tag_name)): Path<(Uuid, String)>,
) -> ApiResult<()> {
    state
        .tag_service
        .remove_tag_from_problem(problem_id, &tag_name)
        .await?;
    let response = ApiResponse::success_message(format!(
        "Tag '{}' removed from problem successfully",
        tag_name
    ));
    Ok(Json(response))
}

pub async fn get_tags_for_problem(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
) -> ApiResult<Vec<TagResponse>> {
    let tags = state.tag_service.get_tags_for_problem(problem_id).await?;

    let tag_responses: Vec<TagResponse> = tags.into_iter().map(|tag| tag.into()).collect();

    let response = ApiResponse::success(tag_responses);
    Ok(Json(response))
}

pub async fn get_popular_tags(
    State(state): State<AppState>,
    Query(query): Query<PopularTagsQuery>,
) -> ApiResult<Vec<TagWithCountResponse>> {
    let tags = state.tag_service.get_popular_tags(query.limit).await?;

    let tag_responses: Vec<TagWithCountResponse> = tags.into_iter().map(|tag| tag.into()).collect();

    let response = ApiResponse::success(tag_responses);
    Ok(Json(response))
}

pub async fn bulk_add_tags_to_problem(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
    Json(request): Json<BulkTagRequest>,
) -> ApiResult<()> {
    state
        .tag_service
        .bulk_add_tags_to_problem(problem_id, request.tag_names)
        .await?;
    let response = ApiResponse::success_message("Tags added to problem successfully".to_string());
    Ok(Json(response))
}

pub async fn replace_problem_tags(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
    Json(request): Json<BulkTagRequest>,
) -> ApiResult<()> {
    state
        .tag_service
        .replace_problem_tags(problem_id, request.tag_names)
        .await?;
    let response = ApiResponse::success_message("Problem tags updated successfully".to_string());
    Ok(Json(response))
}
