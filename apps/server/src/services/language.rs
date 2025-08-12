use async_trait::async_trait;

use crate::errors::Result;
use crate::models::language::Language;

#[async_trait]
pub trait LanguageServiceTrait: Send + Sync {
    async fn get_all_languages(&self) -> Result<Vec<Language>>;
    async fn get_language_by_name(&self, name: &str) -> Result<Option<Language>>;
    async fn is_language_supported(&self, name: &str) -> Result<bool>;
}
