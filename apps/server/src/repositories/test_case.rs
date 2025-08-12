use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::Result, models::test_case::*};

#[async_trait]
pub trait TestCaseRepositoryTrait: Send + Sync {
    async fn create(&self, test_case: CreateTestCaseRequest) -> Result<TestCase>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TestCase>>;
    async fn find_by_problem(
        &self,
        problem_id: Uuid,
        include_hidden: bool,
    ) -> Result<Vec<TestCase>>;
    async fn find_sample_cases(&self, problem_id: Uuid) -> Result<Vec<TestCase>>;
    async fn find_hidden_cases(&self, problem_id: Uuid) -> Result<Vec<TestCase>>;
    async fn update(&self, id: Uuid, update: UpdateTestCaseRequest) -> Result<TestCase>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn count_by_problem(&self, problem_id: Uuid) -> Result<i64>;
    async fn count_sample_cases(&self, problem_id: Uuid) -> Result<i64>;
    async fn bulk_create(&self, test_cases: Vec<CreateTestCaseRequest>) -> Result<Vec<TestCase>>;
    async fn delete_by_problem(&self, problem_id: Uuid) -> Result<()>;
}

pub struct TestCaseRepository {
    pool: PgPool,
}

impl TestCaseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TestCaseRepositoryTrait for TestCaseRepository {
    async fn create(&self, test_case: CreateTestCaseRequest) -> Result<TestCase> {
        let query = r#"
            INSERT INTO test_cases (problem_id, input_data, expected_output, is_sample)
            VALUES ($1, $2, $3, $4)
            RETURNING id, problem_id, input_data, expected_output, is_sample, created_at
        "#;

        let test_case = sqlx::query_as::<_, TestCase>(query)
            .bind(&test_case.problem_id)
            .bind(&test_case.input_data)
            .bind(&test_case.expected_output)
            .bind(&test_case.is_sample)
            .fetch_one(&self.pool)
            .await?;

        Ok(test_case)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TestCase>> {
        let query = "SELECT * FROM test_cases WHERE id = $1";

        let test_case = sqlx::query_as::<_, TestCase>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(test_case)
    }

    async fn find_by_problem(
        &self,
        problem_id: Uuid,
        include_hidden: bool,
    ) -> Result<Vec<TestCase>> {
        let query = if include_hidden {
            "SELECT * FROM test_cases WHERE problem_id = $1 ORDER BY is_sample DESC, created_at ASC"
        } else {
            "SELECT * FROM test_cases WHERE problem_id = $1 AND is_sample = true ORDER BY created_at ASC"
        };

        let test_cases = sqlx::query_as::<_, TestCase>(query)
            .bind(problem_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(test_cases)
    }

    async fn find_sample_cases(&self, problem_id: Uuid) -> Result<Vec<TestCase>> {
        let query = r#"
            SELECT * FROM test_cases 
            WHERE problem_id = $1 AND is_sample = true 
            ORDER BY created_at ASC
        "#;

        let test_cases = sqlx::query_as::<_, TestCase>(query)
            .bind(problem_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(test_cases)
    }

    async fn find_hidden_cases(&self, problem_id: Uuid) -> Result<Vec<TestCase>> {
        let query = r#"
            SELECT * FROM test_cases 
            WHERE problem_id = $1 AND is_sample = false 
            ORDER BY created_at ASC
        "#;

        let test_cases = sqlx::query_as::<_, TestCase>(query)
            .bind(problem_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(test_cases)
    }

    async fn update(&self, id: Uuid, update: UpdateTestCaseRequest) -> Result<TestCase> {
        let query = r#"
        UPDATE test_cases
        SET input_data = COALESCE($2, input_data),
            expected_output = COALESCE($3, expected_output),
            is_sample = COALESCE($4, is_sample)
        WHERE id = $1
        RETURNING *
        "#;

        let test_case = sqlx::query_as::<_, TestCase>(query)
            .bind(id)
            .bind(&update.input_data)
            .bind(&update.expected_output)
            .bind(&update.is_sample)
            .fetch_one(&self.pool)
            .await?;

        Ok(test_case)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let query = "DELETE FROM test_cases WHERE id = $1";

        sqlx::query(query).bind(id).execute(&self.pool).await?;

        Ok(())
    }

    async fn count_by_problem(&self, problem_id: Uuid) -> Result<i64> {
        let query = r#"SELECT COUNT(*) as count from test_cases where problem_id = $1"#;

        let row: (i64,) = sqlx::query_as(query)
            .bind(problem_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }

    async fn count_sample_cases(&self, problem_id: Uuid) -> Result<i64> {
        let query = r#"
            SELECT COUNT(*) as count FROM test_cases 
            WHERE problem_id = $1 AND is_sample = true
        "#;

        let row: (i64,) = sqlx::query_as(query)
            .bind(problem_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }

    async fn bulk_create(&self, test_cases: Vec<CreateTestCaseRequest>) -> Result<Vec<TestCase>> {
        if test_cases.is_empty() {
            return Ok(Vec::new());
        }

        let mut query = String::from(
            r#"
            INSERT INTO test_cases (problem_id, input_data, expected_outpjut, is_sample)
            VALUES
        "#,
        );

        let mut values = Vec::new();

        for (i, _) in test_cases.iter().enumerate() {
            let base = i * 4;
            values.push(format!(
                "(${}, ${}, ${}, ${})",
                base + 1,
                base + 2,
                base + 3,
                base + 4
            ));
        }

        query.push_str(&values.join(", "));
        query.push_str(" RETURNING *");

        let mut query_builder = sqlx::query_as::<_, TestCase>(&query);
        for test_case in &test_cases {
            query_builder = query_builder
                .bind(&test_case.problem_id)
                .bind(&test_case.input_data)
                .bind(&test_case.expected_output)
                .bind(&test_case.is_sample);
        }

        let created_test_cases = query_builder.fetch_all(&self.pool).await?;
        Ok(created_test_cases)
    }

    async fn delete_by_problem(&self, problem_id: Uuid) -> Result<()> {
        let query = "DELETE FROM test_cases WHERE problem_id = $1";

        sqlx::query(query)
            .bind(problem_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
