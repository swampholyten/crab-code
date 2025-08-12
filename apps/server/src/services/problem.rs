use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::{Result, ServiceError},
    models::problem::{
        CreateProblemRequest, DifficultyLevel, Problem, ProblemFilter, UpdateProblemRequest,
    },
    repositories::problem::ProblemRepositoryTrait,
};

#[async_trait]
pub trait ProblemServiceTrait: Send + Sync {
    async fn create_problem(&self, request: CreateProblemRequest) -> Result<Problem>;
    async fn get_problem_by_id(&self, id: Uuid) -> Result<Option<Problem>>;
    async fn get_problem_by_slug(&self, slug: &str) -> Result<Option<Problem>>;
    async fn list_problems(&self, filter: ProblemFilter) -> Result<Vec<Problem>>;
    async fn update_problem(&self, id: Uuid, update: UpdateProblemRequest) -> Result<Problem>;
    async fn delete_problem(&self, id: Uuid) -> Result<()>;
    async fn search_problems(&self, query: &str) -> Result<Vec<Problem>>;
    async fn count_problems(&self) -> Result<i64>;
    async fn get_problems_by_difficulty(&self, difficulty: DifficultyLevel)
    -> Result<Vec<Problem>>;
}

#[derive(Clone)]
pub struct ProblemService {
    problem_repository: Arc<dyn ProblemRepositoryTrait + Send + Sync>,
}

impl ProblemService {
    pub fn new(problem_repository: Arc<dyn ProblemRepositoryTrait + Send + Sync>) -> Self {
        Self { problem_repository }
    }
}

#[async_trait]
impl ProblemServiceTrait for ProblemService {
    async fn create_problem(&self, request: CreateProblemRequest) -> Result<Problem> {
        // Validation
        if request.title.trim().is_empty() {
            return Err(ServiceError::ValidationError("Title cannot be empty".to_string()).into());
        }

        if request.slug.trim().is_empty() {
            return Err(ServiceError::ValidationError("Slug cannot be empty".to_string()).into());
        }

        if request.description.trim().is_empty() {
            return Err(
                ServiceError::ValidationError("Description cannot be empty".to_string()).into(),
            );
        }

        // Check if slug already exists
        if self
            .problem_repository
            .find_by_slug(&request.slug)
            .await?
            .is_some()
        {
            return Err(ServiceError::ConflictError(
                "A problem with this slug already exists".to_string(),
            )
            .into());
        }

        // Validate slug format (alphanumeric and hyphens only)
        if !request
            .slug
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-')
        {
            return Err(ServiceError::ValidationError(
                "Slug can only contain alphanumeric characters and hyphens".to_string(),
            )
            .into());
        }

        let problem = self.problem_repository.create(request).await?;
        Ok(problem)
    }

    async fn get_problem_by_id(&self, id: Uuid) -> Result<Option<Problem>> {
        self.problem_repository.find_by_id(id).await
    }

    async fn get_problem_by_slug(&self, slug: &str) -> Result<Option<Problem>> {
        self.problem_repository.find_by_slug(slug).await
    }

    async fn list_problems(&self, filter: ProblemFilter) -> Result<Vec<Problem>> {
        self.problem_repository.list(filter).await
    }

    async fn update_problem(&self, id: Uuid, update: UpdateProblemRequest) -> Result<Problem> {
        // Check if problem exists
        if self.problem_repository.find_by_id(id).await?.is_none() {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        // Validate fields if provided
        if let Some(ref title) = update.title
            && title.trim().is_empty()
        {
            return Err(ServiceError::ValidationError("Title cannot be empty".to_string()).into());
        }

        if let Some(ref slug) = update.slug {
            if slug.trim().is_empty() {
                return Err(
                    ServiceError::ValidationError("Slug cannot be empty".to_string()).into(),
                );
            }

            // Check if new slug conflicts with existing problems (excluding current one)
            if let Some(existing) = self.problem_repository.find_by_slug(slug).await?
                && existing.id != id
            {
                return Err(ServiceError::ConflictError(
                    "A problem with this slug already exists".to_string(),
                )
                .into());
            }

            // Validate slug format
            if !slug.chars().all(|c| c.is_alphanumeric() || c == '-') {
                return Err(ServiceError::ValidationError(
                    "Slug can only contain alphanumeric characters and hyphens".to_string(),
                )
                .into());
            }
        }

        if let Some(ref description) = update.description
            && description.trim().is_empty()
        {
            return Err(
                ServiceError::ValidationError("Description cannot be empty".to_string()).into(),
            );
        }

        self.problem_repository.update(id, update).await
    }

    async fn delete_problem(&self, id: Uuid) -> Result<()> {
        // Check if problem exists
        if self.problem_repository.find_by_id(id).await?.is_none() {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        self.problem_repository.delete(id).await
    }

    async fn search_problems(&self, query: &str) -> Result<Vec<Problem>> {
        if query.trim().is_empty() {
            return Err(
                ServiceError::ValidationError("Search query cannot be empty".to_string()).into(),
            );
        }

        self.problem_repository.search(query).await
    }

    async fn count_problems(&self) -> Result<i64> {
        self.problem_repository.count().await
    }

    async fn get_problems_by_difficulty(
        &self,
        difficulty: DifficultyLevel,
    ) -> Result<Vec<Problem>> {
        self.problem_repository.find_by_difficulty(difficulty).await
    }
}
