use async_trait::async_trait;
use sqlx::PgPool;

use crate::errors::Result;
use crate::models::language::*;

#[async_trait]
pub trait LanguageRepositoryTrait: Send + Sync {
    async fn create(&self, language: CreateLanguageRequest) -> Result<Language>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Language>>;
    async fn list(&self) -> Result<Vec<Language>>;
    async fn update(&self, name: &str, update: UpdateLanguageRequest) -> Result<Language>;
    async fn delete(&self, name: &str) -> Result<()>;
    async fn exists(&self, name: &str) -> Result<bool>;
}

pub struct LanguageRepository {
    pool: PgPool,
}

impl LanguageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LanguageRepositoryTrait for LanguageRepository {
    async fn create(&self, language: CreateLanguageRequest) -> Result<Language> {
        let query = r#"
            INSERT INTO languages (name, display_name, file_extension)
            VALUES ($1, $2, $3)
            RETURNING name, display_name, file_extension, created_at
        "#;

        let language = sqlx::query_as::<_, Language>(query)
            .bind(&language.name)
            .bind(&language.display_name)
            .bind(&language.file_extension)
            .fetch_one(&self.pool)
            .await?;

        Ok(language)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Language>> {
        let query = "SELECT * FROM languages WHERE name = $1";

        let language = sqlx::query_as::<_, Language>(query)
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(language)
    }

    async fn list(&self) -> Result<Vec<Language>> {
        let query = "SELECT * FROM languages ORDER BY display_name ASC";

        let languages = sqlx::query_as::<_, Language>(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(languages)
    }

    async fn update(&self, name: &str, update: UpdateLanguageRequest) -> Result<Language> {
        let query = r#"
            UPDATE languages 
            SET display_name = COALESCE($2, display_name),
                file_extension = COALESCE($3, file_extension)
            WHERE name = $1
            RETURNING *
        "#;

        let language = sqlx::query_as::<_, Language>(query)
            .bind(name)
            .bind(&update.display_name)
            .bind(&update.file_extension)
            .fetch_one(&self.pool)
            .await?;

        Ok(language)
    }

    async fn delete(&self, name: &str) -> Result<()> {
        let query = "DELETE FROM languages WHERE name = $1";

        sqlx::query(query).bind(name).execute(&self.pool).await?;

        Ok(())
    }

    async fn exists(&self, name: &str) -> Result<bool> {
        let query = "SELECT EXISTS(SELECT 1 FROM languages WHERE name = $1)";

        let exists: (bool,) = sqlx::query_as(query)
            .bind(name)
            .fetch_one(&self.pool)
            .await?;

        Ok(exists.0)
    }
}
