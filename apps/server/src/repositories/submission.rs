use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::Result, models::submission::*};

#[async_trait]
pub trait SubmissionRepositoryTrait: Send + Sync {
    async fn create(&self, submission: CreateSubmissionRequest) -> Result<Submission>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Submission>>;
    async fn find_by_user(
        &self,
        user_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>>;
    async fn find_by_problem(
        &self,
        problem_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>>;
    async fn update_status(
        &self,
        id: Uuid,
        status: SubmissionStatus,
        execution_time: Option<i32>,
        memory_used: Option<i32>,
        error_message: Option<String>,
    ) -> Result<()>;
    async fn find_latest_accepted(
        &self,
        user_id: Uuid,
        problem_id: Uuid,
    ) -> Result<Option<Submission>>;
    async fn count_by_status(&self, status: SubmissionStatus) -> Result<i64>;
}

pub struct SubmissionRepository {
    pool: PgPool,
}

impl SubmissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SubmissionRepositoryTrait for SubmissionRepository {
    async fn create(&self, submission: CreateSubmissionRequest) -> Result<Submission> {
        let query = r#"
            INSERT INTO submissions (user_id, problem_id, language_id, code, status)
            VALUES ($1, $2, $3, $4, 'wrong_answer')
            RETURNING id, user_id, problem_id, language_id, code, status, 
                      execution_time, memory_used, error_message, created_at
        "#;

        let submission = sqlx::query_as::<_, Submission>(query)
            .bind(submission.user_id)
            .bind(submission.problem_id)
            .bind(&submission.language_id)
            .bind(&submission.code)
            .fetch_one(&self.pool)
            .await?;

        Ok(submission)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Submission>> {
        let query = "SELECT * FROM submissions WHERE id = $1";

        let submission = sqlx::query_as::<_, Submission>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(submission)
    }

    async fn find_by_user(
        &self,
        user_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>> {
        let mut query = String::from("SELECT * FROM submissions WHERE user_id = $1");
        let mut param_count = 1;

        if filter.status.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND status = ${}", param_count));
        }

        if filter.language_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND language_id = ${}", param_count));
        }

        query.push_str(" ORDER BY created_at DESC");

        if filter.limit.is_some() {
            param_count += 1;
            query.push_str(&format!(" LIMIT ${}", param_count));
        }

        if filter.offset.is_some() {
            param_count += 1;
            query.push_str(&format!(" OFFSET ${}", param_count));
        }

        let mut query_builder = sqlx::query_as::<_, Submission>(&query).bind(user_id);

        if let Some(status) = &filter.status {
            query_builder = query_builder.bind(status);
        }

        if let Some(language_id) = &filter.language_id {
            query_builder = query_builder.bind(language_id);
        }

        if let Some(limit) = filter.limit {
            query_builder = query_builder.bind(limit);
        }

        if let Some(offset) = filter.offset {
            query_builder = query_builder.bind(offset);
        }

        let submissions = query_builder.fetch_all(&self.pool).await?;
        Ok(submissions)
    }

    async fn find_by_problem(
        &self,
        problem_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>> {
        let mut query = String::from("SELECT * FROM submissions WHERE problem_id = $1");
        let mut param_count = 1;

        if filter.status.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND status = ${}", param_count));
        }

        if filter.language_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND language_id = ${}", param_count));
        }

        query.push_str(" ORDER BY created_at DESC");

        if filter.limit.is_some() {
            param_count += 1;
            query.push_str(&format!(" LIMIT ${}", param_count));
        }

        if filter.offset.is_some() {
            param_count += 1;
            query.push_str(&format!(" OFFSET ${}", param_count));
        }

        let mut query_builder = sqlx::query_as::<_, Submission>(&query).bind(problem_id);

        if let Some(status) = &filter.status {
            query_builder = query_builder.bind(status);
        }

        if let Some(language_id) = &filter.language_id {
            query_builder = query_builder.bind(language_id);
        }

        if let Some(limit) = filter.limit {
            query_builder = query_builder.bind(limit);
        }

        if let Some(offset) = filter.offset {
            query_builder = query_builder.bind(offset);
        }

        let submissions = query_builder.fetch_all(&self.pool).await?;
        Ok(submissions)
    }

    async fn update_status(
        &self,
        id: Uuid,
        status: SubmissionStatus,
        execution_time: Option<i32>,
        memory_used: Option<i32>,
        error_message: Option<String>,
    ) -> Result<()> {
        let query = r#"
            UPDATE submissions 
            SET status = $2,
                execution_time = $3,
                memory_used = $4,
                error_message = $5
            WHERE id = $1
        "#;

        sqlx::query(query)
            .bind(id)
            .bind(status)
            .bind(execution_time)
            .bind(memory_used)
            .bind(error_message)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_latest_accepted(
        &self,
        user_id: Uuid,
        problem_id: Uuid,
    ) -> Result<Option<Submission>> {
        let query = r#"
            SELECT * FROM submissions 
            WHERE user_id = $1 AND problem_id = $2 AND status = 'accepted'
            ORDER BY created_at DESC 
            LIMIT 1
        "#;

        let submission = sqlx::query_as::<_, Submission>(query)
            .bind(user_id)
            .bind(problem_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(submission)
    }

    async fn count_by_status(&self, status: SubmissionStatus) -> Result<i64> {
        let query = "SELECT COUNT(*) as count FROM submissions WHERE status = $1";

        let row: (i64,) = sqlx::query_as(query)
            .bind(status)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }
}
