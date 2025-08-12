use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::{Result, ServiceError},
    models::{
        problem::Problem,
        tag::{CreateTagRequest, Tag, TagWithCount},
    },
    repositories::{problem::ProblemRepositoryTrait, tag::TagRepositoryTrait},
};

#[async_trait]
pub trait TagServiceTrait: Send + Sync {
    async fn get_all_tags(&self) -> Result<Vec<Tag>>;
    async fn create_tag(&self, request: CreateTagRequest) -> Result<Tag>;
    async fn get_tag_by_name(&self, name: &str) -> Result<Option<Tag>>;
    async fn delete_tag(&self, name: &str) -> Result<()>;
    async fn get_problems_by_tag(&self, tag_name: &str) -> Result<Vec<Problem>>;
    async fn add_tag_to_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()>;
    async fn remove_tag_from_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()>;
    async fn get_tags_for_problem(&self, problem_id: Uuid) -> Result<Vec<Tag>>;
    async fn get_popular_tags(&self, limit: Option<i32>) -> Result<Vec<TagWithCount>>;
    async fn bulk_add_tags_to_problem(
        &self,
        problem_id: Uuid,
        tag_names: Vec<String>,
    ) -> Result<()>;
    async fn replace_problem_tags(&self, problem_id: Uuid, tag_names: Vec<String>) -> Result<()>;
}

#[derive(Clone)]
pub struct TagService {
    tag_repository: Arc<dyn TagRepositoryTrait + Send + Sync>,
    problem_repository: Arc<dyn ProblemRepositoryTrait + Send + Sync>,
}

impl TagService {
    pub fn new(
        tag_repository: Arc<dyn TagRepositoryTrait + Send + Sync>,
        problem_repository: Arc<dyn ProblemRepositoryTrait + Send + Sync>,
    ) -> Self {
        Self {
            tag_repository,
            problem_repository,
        }
    }
}

#[async_trait]
impl TagServiceTrait for TagService {
    async fn get_all_tags(&self) -> Result<Vec<Tag>> {
        self.tag_repository.list().await
    }

    async fn create_tag(&self, request: CreateTagRequest) -> Result<Tag> {
        // Validation
        if request.name.trim().is_empty() {
            return Err(
                ServiceError::ValidationError("Tag name cannot be empty".to_string()).into(),
            );
        }

        // Validate tag name format (lowercase, alphanumeric, hyphens, underscores)
        if !request
            .name
            .chars()
            .all(|c| c.is_lowercase() && (c.is_alphanumeric() || c == '-' || c == '_'))
        {
            return Err(ServiceError::ValidationError(
                "Tag name must be lowercase and contain only alphanumeric characters, hyphens, or underscores".to_string(),
            ).into());
        }

        // Check if tag already exists
        if self.tag_repository.exists(&request.name).await? {
            return Err(ServiceError::ConflictError(format!(
                "Tag '{}' already exists",
                request.name
            ))
            .into());
        }

        let tag = self.tag_repository.create(request).await?;
        Ok(tag)
    }

    async fn get_tag_by_name(&self, name: &str) -> Result<Option<Tag>> {
        self.tag_repository.find_by_name(name).await
    }

    async fn delete_tag(&self, name: &str) -> Result<()> {
        // Check if tag exists
        if !self.tag_repository.exists(name).await? {
            return Err(ServiceError::NotFoundError(format!("Tag '{}' not found", name)).into());
        }

        // NOTE: Due to CASCADE DELETE, deleting a tag will automatically remove its associations
        self.tag_repository.delete(name).await
    }

    async fn get_problems_by_tag(&self, tag_name: &str) -> Result<Vec<Problem>> {
        // Check if tag exists
        if !self.tag_repository.exists(tag_name).await? {
            return Err(
                ServiceError::NotFoundError(format!("Tag '{}' not found", tag_name)).into(),
            );
        }

        self.tag_repository.find_problems_by_tag(tag_name).await
    }

    async fn add_tag_to_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()> {
        // Validate problem exists
        if self
            .problem_repository
            .find_by_id(problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        // Validate tag exists
        if !self.tag_repository.exists(tag_name).await? {
            return Err(
                ServiceError::NotFoundError(format!("Tag '{}' not found", tag_name)).into(),
            );
        }

        self.tag_repository
            .add_tag_to_problem(problem_id, tag_name)
            .await
    }

    async fn remove_tag_from_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()> {
        // Validate problem exists
        if self
            .problem_repository
            .find_by_id(problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        // Validate tag exists
        if !self.tag_repository.exists(tag_name).await? {
            return Err(
                ServiceError::NotFoundError(format!("Tag '{}' not found", tag_name)).into(),
            );
        }

        // Check if tag is assigned to problem
        if !self
            .tag_repository
            .is_tag_assigned_to_problem(problem_id, tag_name)
            .await?
        {
            return Err(ServiceError::NotFoundError(format!(
                "Tag '{}' is not assigned to this problem",
                tag_name
            ))
            .into());
        }

        self.tag_repository
            .remove_tag_from_problem(problem_id, tag_name)
            .await
    }

    async fn get_tags_for_problem(&self, problem_id: Uuid) -> Result<Vec<Tag>> {
        // Validate problem exists
        if self
            .problem_repository
            .find_by_id(problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        self.tag_repository.find_tags_for_problem(problem_id).await
    }

    async fn get_popular_tags(&self, limit: Option<i32>) -> Result<Vec<TagWithCount>> {
        // Validate limit if provided
        if let Some(limit) = limit
            && limit <= 0
        {
            return Err(
                ServiceError::ValidationError("Limit must be greater than 0".to_string()).into(),
            );
        }

        self.tag_repository.get_popular_tags(limit).await
    }

    async fn bulk_add_tags_to_problem(
        &self,
        problem_id: Uuid,
        tag_names: Vec<String>,
    ) -> Result<()> {
        // Validate problem exists
        if self
            .problem_repository
            .find_by_id(problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        // Validate all tags exist
        for tag_name in &tag_names {
            if !self.tag_repository.exists(tag_name).await? {
                return Err(
                    ServiceError::NotFoundError(format!("Tag '{}' not found", tag_name)).into(),
                );
            }
        }

        // Add all tags (using ON CONFLICT DO NOTHING to handle duplicates)
        for tag_name in tag_names {
            self.tag_repository
                .add_tag_to_problem(problem_id, &tag_name)
                .await?;
        }

        Ok(())
    }

    async fn replace_problem_tags(&self, problem_id: Uuid, tag_names: Vec<String>) -> Result<()> {
        // Validate problem exists
        if self
            .problem_repository
            .find_by_id(problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        // Validate all new tags exist
        for tag_name in &tag_names {
            if !self.tag_repository.exists(tag_name).await? {
                return Err(
                    ServiceError::NotFoundError(format!("Tag '{}' not found", tag_name)).into(),
                );
            }
        }

        // Get current tags
        let current_tags = self
            .tag_repository
            .find_tags_for_problem(problem_id)
            .await?;
        let current_tag_names: Vec<String> = current_tags.into_iter().map(|t| t.name).collect();

        // Remove tags that are not in the new list
        for current_tag in &current_tag_names {
            if !tag_names.contains(current_tag) {
                self.tag_repository
                    .remove_tag_from_problem(problem_id, current_tag)
                    .await?;
            }
        }

        // Add new tags that are not currently assigned
        for new_tag in &tag_names {
            if !current_tag_names.contains(new_tag) {
                self.tag_repository
                    .add_tag_to_problem(problem_id, new_tag)
                    .await?;
            }
        }

        Ok(())
    }
}
