use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::{
        response::{ApiResponse, ApiResult, PaginatedResponse},
        state::AppState,
    },
    models::user::*,
};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> ApiResult<UserResponse> {
    let create_request = crate::models::user::CreateUserRequest {
        username: request.username,
        email: request.email,
        password_hash: request.password,
        role: UserRole::User,
    };

    let user = state.user_service.create_user(create_request).await?;
    let response =
        ApiResponse::success_with_message(user.into(), "User created successfully".to_string());

    Ok(Json(response))
}

pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<UserResponse> {
    let user = state
        .user_service
        .get_user_by_id(user_id)
        .await?
        .ok_or_else(|| crate::errors::Error::NotFound("User not found".to_string()))?;

    let response = ApiResponse::success(user.into());
    Ok(Json(response))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> ApiResult<UserResponse> {
    let update_request = crate::models::user::UpdateUserRequest {
        username: request.username,
        email: request.email,
        avatar_url: request.avatar_url,
    };

    let user = state
        .user_service
        .update_user(user_id, update_request)
        .await?;
    let response =
        ApiResponse::success_with_message(user.into(), "User updated successfully".to_string());

    Ok(Json(response))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<()> {
    state.user_service.delete_user(user_id).await?;
    let response = ApiResponse::success_message("User deleted successfully".to_string());
    Ok(Json(response))
}

pub async fn list_users(
    State(state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
) -> ApiResult<PaginatedResponse<UserResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    // Get total count (you'll need to add this method to your service)
    let total = state.user_service.count_users().await?;

    // Get users
    let users = state
        .user_service
        .list_users(Some(per_page), Some(offset))
        .await?;

    let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();
    let paginated = PaginatedResponse::new(user_responses, total, page, per_page);

    let response = ApiResponse::success(paginated);
    Ok(Json(response))
}

pub async fn get_user_profile(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<UserProfile> {
    let profile = state.user_service.get_user_profile(user_id).await?;
    let response = ApiResponse::success(profile);
    Ok(Json(response))
}
