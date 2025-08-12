use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::{Result, ServiceError},
    models::submission::{CreateSubmissionRequest, JudgeResult, Submission, SubmissionFilter},
    repositories::{
        problem::ProblemRepositoryTrait, submission::SubmissionRepositoryTrait,
        user::UserRepositoryTrait,
    },
};

#[async_trait]
pub trait SubmissionServiceTrait: Send + Sync {
    async fn create_submission(&self, request: CreateSubmissionRequest) -> Result<Submission>;
    async fn get_submission_by_id(&self, id: Uuid) -> Result<Option<Submission>>;
    async fn get_user_submissions(
        &self,
        user_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>>;
    async fn get_problem_submissions(
        &self,
        problem_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>>;
    async fn update_submission_result(&self, id: Uuid, result: JudgeResult) -> Result<()>;
}

#[derive(Clone)]
pub struct SubmissionService {
    submission_repository: Arc<dyn SubmissionRepositoryTrait + Send + Sync>,
    problem_repository: Arc<dyn ProblemRepositoryTrait + Send + Sync>,
    user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>,
}

impl SubmissionService {
    pub fn new(
        submission_repository: Arc<dyn SubmissionRepositoryTrait + Send + Sync>,
        problem_repository: Arc<dyn ProblemRepositoryTrait + Send + Sync>,
        user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>,
    ) -> Self {
        Self {
            submission_repository,
            problem_repository,
            user_repository,
        }
    }
}

#[async_trait]
impl SubmissionServiceTrait for SubmissionService {
    async fn create_submission(&self, request: CreateSubmissionRequest) -> Result<Submission> {
        // Validate user exists
        if self
            .user_repository
            .find_by_id(request.user_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("User not found".to_string()).into());
        }

        // Validate problem exists
        if self
            .problem_repository
            .find_by_id(request.problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        // Validate code is not empty
        if request.code.trim().is_empty() {
            return Err(ServiceError::ValidationError("Code cannot be empty".to_string()).into());
        }

        // Validate code size (e.g., max 64KB)
        if request.code.len() > 65536 {
            return Err(
                ServiceError::ValidationError("Code size exceeds limit".to_string()).into(),
            );
        }

        // Create submission
        let submission = self.submission_repository.create(request).await?;

        Ok(submission)
    }

    async fn get_submission_by_id(&self, id: Uuid) -> Result<Option<Submission>> {
        self.submission_repository.find_by_id(id).await
    }

    async fn get_user_submissions(
        &self,
        user_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>> {
        // Validate user exists
        if self.user_repository.find_by_id(user_id).await?.is_none() {
            return Err(ServiceError::NotFoundError("User not found".to_string()).into());
        }

        self.submission_repository
            .find_by_user(user_id, filter)
            .await
    }

    async fn get_problem_submissions(
        &self,
        problem_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>> {
        // Validate problem exists
        if self
            .problem_repository
            .find_by_id(problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        self.submission_repository
            .find_by_problem(problem_id, filter)
            .await
    }

    async fn update_submission_result(&self, id: Uuid, result: JudgeResult) -> Result<()> {
        // Validate submission exists
        if self.submission_repository.find_by_id(id).await?.is_none() {
            return Err(ServiceError::NotFoundError("Submission not found".to_string()).into());
        }

        self.submission_repository
            .update_status(
                id,
                result.status,
                result.execution_time,
                result.memory_used,
                result.error_message,
            )
            .await?;

        Ok(())
    }
}
