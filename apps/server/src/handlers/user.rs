use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{common::state::AppState, errors::Result, models::user::*};

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
    pub limit: Option<i32>,
    pub offset: Option<i32>,
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

#[derive(Debug, Serialize)]
pub struct ListUsersResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
}

// Handler functions
pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>> {
    let create_request = crate::models::user::CreateUserRequest {
        username: request.username,
        email: request.email,
        password_hash: request.password, // Will be hashed in service
        role: UserRole::User,
    };

    let user = state.user_service.create_user(create_request).await?;
    Ok(Json(user.into()))
}

pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>> {
    let user = state
        .user_service
        .get_user_by_id(user_id)
        .await?
        .ok_or_else(|| crate::errors::Error::NotFound("User not found".to_string()))?;

    Ok(Json(user.into()))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>> {
    let update_request = crate::models::user::UpdateUserRequest {
        username: request.username,
        email: request.email,
        avatar_url: request.avatar_url,
    };

    let user = state
        .user_service
        .update_user(user_id, update_request)
        .await?;
    Ok(Json(user.into()))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode> {
    state.user_service.delete_user(user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_users(
    State(state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<ListUsersResponse>> {
    let users = state
        .user_service
        .list_users(query.limit, query.offset)
        .await?;

    let response = ListUsersResponse {
        total: users.len(),
        users: users.into_iter().map(|u| u.into()).collect(),
    };

    Ok(Json(response))
}

pub async fn get_user_profile(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserProfile>> {
    let profile = state.user_service.get_user_profile(user_id).await?;
    Ok(Json(profile))
}
