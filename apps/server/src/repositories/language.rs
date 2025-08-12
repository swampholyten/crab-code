use crate::errors::Result;
use crate::models::language::*;

pub trait LanguageRepositoryTrait: Send + Sync {
    async fn create(&self, language: CreateLanguageRequest) -> Result<Language>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Language>>;
    async fn list(&self) -> Result<Vec<Language>>;
    async fn update(&self, name: &str, update: UpdateLanguageRequest) -> Result<Language>;
    async fn delete(&self, name: &str) -> Result<()>;
}
