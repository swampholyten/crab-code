use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    common::hash::hash_password,
    errors::{Result, ServiceError},
    models::{
        submission::{SubmissionFilter, SubmissionStatus},
        user::{CreateUserRequest, UpdateUserRequest, User, UserProfile},
    },
    repositories::{submission::SubmissionRepositoryTrait, user::UserRepositoryTrait},
};

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn list_users(&self, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<User>>;
    async fn update_user(&self, id: Uuid, update: UpdateUserRequest) -> Result<User>;
    async fn delete_user(&self, id: Uuid) -> Result<()>;
    async fn get_user_profile(&self, id: Uuid) -> Result<UserProfile>;
    async fn count_users(&self) -> Result<i64>;
}

#[derive(Clone)]
pub struct UserService {
    pub user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>,
    submission_repository: Arc<dyn SubmissionRepositoryTrait + Send + Sync>,
}

impl UserService {
    pub fn new(
        user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>,
        submission_repository: Arc<dyn SubmissionRepositoryTrait + Send + Sync>,
    ) -> Self {
        Self {
            user_repository,
            submission_repository,
        }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn create_user(&self, mut request: CreateUserRequest) -> Result<User> {
        // Validation
        if request.username.len() < 3 {
            return Err(ServiceError::ValidationError(
                "Username must be at least 3 characters".to_string(),
            )
            .into());
        }

        if request.email.is_empty() || !request.email.contains('@') {
            return Err(ServiceError::ValidationError("Invalid email format".to_string()).into());
        }

        // Check if user already exists
        if self
            .user_repository
            .find_by_email(&request.email)
            .await?
            .is_some()
        {
            return Err(ServiceError::ConflictError("Email already exists".to_string()).into());
        }

        if self
            .user_repository
            .find_by_username(&request.username)
            .await?
            .is_some()
        {
            return Err(ServiceError::ConflictError("Username already exists".to_string()).into());
        }

        // Hash password
        request.password_hash = hash_password(&request.password_hash)
            .map_err(|e| ServiceError::InternalError(e.to_string()))?;

        let user = self.user_repository.create(request).await?;
        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        self.user_repository.find_by_id(id).await
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        self.user_repository.find_by_email(email).await
    }

    async fn list_users(&self, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<User>> {
        self.user_repository.list(limit, offset).await
    }

    async fn update_user(&self, id: Uuid, update: UpdateUserRequest) -> Result<User> {
        // Check if user exists
        if self.user_repository.find_by_id(id).await?.is_none() {
            return Err(ServiceError::NotFoundError("User not found".to_string()).into());
        }

        // TODO: add better validation
        if let Some(ref username) = update.username
            && username.len() < 3
        {
            return Err(ServiceError::ValidationError(
                "Username must be at least 3 characters".to_string(),
            )
            .into());
        }

        self.user_repository.update(id, update).await
    }

    async fn delete_user(&self, id: Uuid) -> Result<()> {
        if self.user_repository.find_by_id(id).await?.is_none() {
            return Err(ServiceError::NotFoundError("User not found".to_string()).into());
        }

        self.user_repository.delete(id).await // Direct return
    }

    async fn get_user_profile(&self, id: Uuid) -> Result<UserProfile> {
        let user = self
            .user_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| ServiceError::NotFoundError("User not found".to_string()))?;

        // Get user statistics from submissions
        let submissions = self
            .submission_repository
            .find_by_user(id, SubmissionFilter::default())
            .await?;

        let total_submissions = submissions.len() as i32;
        let accepted_submissions = submissions
            .iter()
            .filter(|s| s.status == SubmissionStatus::Accepted)
            .count() as i32;

        let profile = UserProfile {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            role: user.role,
            total_submissions,
            accepted_submissions,
            created_at: user.created_at,
        };

        Ok(profile)
    }

    async fn count_users(&self) -> Result<i64> {
        self.user_repository.count().await
    }
}
