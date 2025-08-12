use std::sync::Arc;

use async_trait::async_trait;

use crate::errors::{Result, ServiceError};
use crate::models::language::{CreateLanguageRequest, Language, UpdateLanguageRequest};
use crate::repositories::language::LanguageRepositoryTrait;

#[async_trait]
pub trait LanguageServiceTrait: Send + Sync {
    async fn create_language(&self, request: CreateLanguageRequest) -> Result<Language>;
    async fn get_language_by_name(&self, name: &str) -> Result<Option<Language>>;
    async fn get_all_languages(&self) -> Result<Vec<Language>>;
    async fn update_language(&self, name: &str, update: UpdateLanguageRequest) -> Result<Language>;
    async fn delete_language(&self, name: &str) -> Result<()>;
    async fn is_language_supported(&self, name: &str) -> Result<bool>;
    async fn get_supported_languages(&self) -> Result<Vec<String>>;
}

#[derive(Clone)]
pub struct LanguageService {
    language_repository: Arc<dyn LanguageRepositoryTrait + Send + Sync>,
}

impl LanguageService {
    pub fn new(language_repository: Arc<dyn LanguageRepositoryTrait + Send + Sync>) -> Self {
        Self {
            language_repository,
        }
    }
}

#[async_trait]
impl LanguageServiceTrait for LanguageService {
    async fn create_language(&self, request: CreateLanguageRequest) -> Result<Language> {
        // Validation
        if request.name.trim().is_empty() {
            return Err(
                ServiceError::ValidationError("Language name cannot be empty".to_string()).into(),
            );
        }

        if request.display_name.trim().is_empty() {
            return Err(
                ServiceError::ValidationError("Display name cannot be empty".to_string()).into(),
            );
        }

        if request.file_extension.trim().is_empty() {
            return Err(ServiceError::ValidationError(
                "File extension cannot be empty".to_string(),
            )
            .into());
        }

        // Validate name format (lowercase, alphanumeric, underscores, hyphens)
        if !request
            .name
            .chars()
            .all(|c| c.is_lowercase() && (c.is_alphanumeric() || c == '_' || c == '-'))
        {
            return Err(ServiceError::ValidationError(
                "Language name must be lowercase and contain only alphanumeric characters, underscores, or hyphens".to_string(),
            ).into());
        }

        // Validate file extension format
        if !request.file_extension.starts_with('.') {
            return Err(ServiceError::ValidationError(
                "File extension must start with a dot (e.g., .py, .js)".to_string(),
            )
            .into());
        }

        // Check if language already exists
        if self.language_repository.exists(&request.name).await? {
            return Err(ServiceError::ConflictError(format!(
                "Language '{}' already exists",
                request.name
            ))
            .into());
        }

        let language = self.language_repository.create(request).await?;
        Ok(language)
    }

    async fn get_language_by_name(&self, name: &str) -> Result<Option<Language>> {
        self.language_repository.find_by_name(name).await
    }

    async fn get_all_languages(&self) -> Result<Vec<Language>> {
        self.language_repository.list().await
    }

    async fn update_language(&self, name: &str, update: UpdateLanguageRequest) -> Result<Language> {
        // Check if language exists
        if !self.language_repository.exists(name).await? {
            return Err(
                ServiceError::NotFoundError(format!("Language '{}' not found", name)).into(),
            );
        }

        // Validate fields if provided
        if let Some(ref display_name) = update.display_name
            && display_name.trim().is_empty()
        {
            return Err(
                ServiceError::ValidationError("Display name cannot be empty".to_string()).into(),
            );
        }

        if let Some(ref file_extension) = update.file_extension {
            if file_extension.trim().is_empty() {
                return Err(ServiceError::ValidationError(
                    "File extension cannot be empty".to_string(),
                )
                .into());
            }

            if !file_extension.starts_with('.') {
                return Err(ServiceError::ValidationError(
                    "File extension must start with a dot (e.g., .py, .js)".to_string(),
                )
                .into());
            }
        }

        self.language_repository.update(name, update).await
    }

    async fn delete_language(&self, name: &str) -> Result<()> {
        // Check if language exists
        if !self.language_repository.exists(name).await? {
            return Err(
                ServiceError::NotFoundError(format!("Language '{}' not found", name)).into(),
            );
        }

        self.language_repository.delete(name).await
    }

    async fn is_language_supported(&self, name: &str) -> Result<bool> {
        self.language_repository.exists(name).await
    }

    async fn get_supported_languages(&self) -> Result<Vec<String>> {
        let languages = self.language_repository.list().await?;
        Ok(languages.into_iter().map(|lang| lang.name).collect())
    }
}
