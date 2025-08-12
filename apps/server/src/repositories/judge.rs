use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::Result;
use crate::models::judge::*;
use crate::repositories::language::LanguageRepositoryTrait;

#[async_trait]
pub trait JudgeRepositoryTrait: Send + Sync {
    async fn create_execution_log(&self, log: CreateExecutionLogRequest) -> Result<ExecutionLog>;
    async fn find_execution_logs_by_submission(
        &self,
        submission_id: Uuid,
    ) -> Result<Vec<ExecutionLog>>;
    async fn get_language_config(&self, language: &str) -> Result<Option<LanguageConfig>>;
}

pub struct JudgeRepository {
    pool: PgPool,
    language_repository: Arc<dyn LanguageRepositoryTrait + Send + Sync>,
}

impl JudgeRepository {
    pub fn new(
        pool: PgPool,
        language_repository: Arc<dyn LanguageRepositoryTrait + Send + Sync>,
    ) -> Self {
        Self {
            pool,
            language_repository,
        }
    }
}

#[async_trait]
impl JudgeRepositoryTrait for JudgeRepository {
    async fn create_execution_log(&self, log: CreateExecutionLogRequest) -> Result<ExecutionLog> {
        let query = r#"
            INSERT INTO execution_logs (
                submission_id, language, execution_time, memory_used, 
                exit_code, stdout, stderr, status, error_message
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING 
                id, submission_id, language, execution_time, memory_used,
                exit_code, stdout, stderr, status, error_message, created_at
        "#;

        let execution_log = sqlx::query_as::<_, ExecutionLog>(query)
            .bind(&log.submission_id)
            .bind(&log.language)
            .bind(&log.execution_time)
            .bind(&log.memory_used)
            .bind(&log.exit_code)
            .bind(&log.stdout)
            .bind(&log.stderr)
            .bind(&log.status)
            .bind(&log.error_message)
            .fetch_one(&self.pool)
            .await?;

        Ok(execution_log)
    }

    async fn find_execution_logs_by_submission(
        &self,
        submission_id: Uuid,
    ) -> Result<Vec<ExecutionLog>> {
        let query = r#"
            SELECT * FROM execution_logs 
            WHERE submission_id = $1 
            ORDER BY created_at DESC
        "#;

        let logs = sqlx::query_as::<_, ExecutionLog>(query)
            .bind(submission_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(logs)
    }

    async fn get_language_config(&self, language: &str) -> Result<Option<LanguageConfig>> {
        // Get language from database
        if let Some(lang) = self.language_repository.find_by_name(language).await? {
            Ok(Some(LanguageConfig::from(lang)))
        } else {
            Ok(None)
        }
    }
}
