use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::Result, models::problem::*};

#[async_trait]
pub trait ProblemRepositoryTrait: Send + Sync {
    async fn create(&self, problem: CreateProblemRequest) -> Result<Problem>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Problem>>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Problem>>;
    async fn update(&self, id: Uuid, update: UpdateProblemRequest) -> Result<Problem>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn list(&self, filter: ProblemFilter) -> Result<Vec<Problem>>;
    async fn find_by_difficulty(&self, difficulty: DifficultyLevel) -> Result<Vec<Problem>>;
    async fn search(&self, query: &str) -> Result<Vec<Problem>>;
    async fn count(&self) -> Result<i64>;
}

pub struct ProblemRepository {
    pool: PgPool,
}

impl ProblemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProblemRepositoryTrait for ProblemRepository {
    async fn create(&self, problem: CreateProblemRequest) -> Result<Problem> {
        let query = r#"
            INSERT INTO problems (title, slug, description, difficulty)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, slug, description, difficulty, created_at, updated_at
        "#;

        let problem = sqlx::query_as::<_, Problem>(query)
            .bind(&problem.title)
            .bind(&problem.slug)
            .bind(&problem.description)
            .bind(&problem.difficulty)
            .fetch_one(&self.pool)
            .await?;

        Ok(problem)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Problem>> {
        let query = "SELECT * FROM problems WHERE id = $1";

        let problem = sqlx::query_as::<_, Problem>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(problem)
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<Problem>> {
        let query = "SELECT * FROM problems WHERE slug = $1";

        let problem = sqlx::query_as::<_, Problem>(query)
            .bind(slug)
            .fetch_optional(&self.pool)
            .await?;

        Ok(problem)
    }

    async fn update(&self, id: Uuid, update: UpdateProblemRequest) -> Result<Problem> {
        let query = r#"
            UPDATE problems 
            SET title = COALESCE($2, title),
                slug = COALESCE($3, slug),
                description = COALESCE($4, description),
                difficulty = COALESCE($5, difficulty),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
        "#;

        let problem = sqlx::query_as::<_, Problem>(query)
            .bind(id)
            .bind(&update.title)
            .bind(&update.slug)
            .bind(&update.description)
            .bind(&update.difficulty)
            .fetch_one(&self.pool)
            .await?;

        Ok(problem)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let query = "DELETE FROM problems WHERE id = $1";

        sqlx::query(query).bind(id).execute(&self.pool).await?;

        Ok(())
    }

    async fn list(&self, filter: ProblemFilter) -> Result<Vec<Problem>> {
        let mut query = String::from("SELECT DISTINCT p.* FROM problems p");
        let mut conditions = Vec::new();
        let mut param_count = 0;

        // Join with problem_tags if filtering by tags
        if let Some(tags) = &filter.tags
            && !tags.is_empty()
        {
            query.push_str(" INNER JOIN problem_tags pt ON p.id = pt.problem_id");
        }

        query.push_str(" WHERE 1=1");

        // Add difficulty filter
        if filter.difficulty.is_some() {
            param_count += 1;
            conditions.push(format!(" AND p.difficulty = ${}", param_count));
        }

        // Add tags filter
        if let Some(tags) = &filter.tags
            && !tags.is_empty()
        {
            param_count += 1;
            conditions.push(format!(" AND pt.tag_id = ANY(${})", param_count));
        }

        // Add search filter
        if let Some(search) = &filter.search
            && !search.trim().is_empty()
        {
            param_count += 1;
            conditions.push(format!(
                " AND (p.title ILIKE ${} OR p.description ILIKE ${})",
                param_count, param_count
            ));
        }

        // Add conditions to query
        for condition in conditions {
            query.push_str(&condition);
        }

        query.push_str(" ORDER BY p.created_at DESC");

        // Add pagination
        if filter.limit.is_some() {
            param_count += 1;
            query.push_str(&format!(" LIMIT ${}", param_count));
        }

        if filter.offset.is_some() {
            param_count += 1;
            query.push_str(&format!(" OFFSET ${}", param_count));
        }

        let mut query_builder = sqlx::query_as::<_, Problem>(&query);

        // Bind parameters in the same order they were added
        if let Some(difficulty) = &filter.difficulty {
            query_builder = query_builder.bind(difficulty);
        }

        if let Some(tags) = &filter.tags
            && !tags.is_empty()
        {
            query_builder = query_builder.bind(tags);
        }

        if let Some(search) = &filter.search
            && !search.trim().is_empty()
        {
            let search_pattern = format!("%{}%", search);
            query_builder = query_builder.bind(search_pattern);
        }

        if let Some(limit) = filter.limit {
            query_builder = query_builder.bind(limit);
        }

        if let Some(offset) = filter.offset {
            query_builder = query_builder.bind(offset);
        }

        let problems = query_builder.fetch_all(&self.pool).await?;
        Ok(problems)
    }

    async fn find_by_difficulty(&self, difficulty: DifficultyLevel) -> Result<Vec<Problem>> {
        let query = "SELECT * FROM problems WHERE difficulty = $1 ORDER BY created_at DESC";

        let problems = sqlx::query_as::<_, Problem>(query)
            .bind(difficulty)
            .fetch_all(&self.pool)
            .await?;

        Ok(problems)
    }

    async fn search(&self, query_text: &str) -> Result<Vec<Problem>> {
        let query = r#"
            SELECT * FROM problems 
            WHERE title ILIKE $1 OR description ILIKE $1
            ORDER BY 
                CASE 
                    WHEN title ILIKE $1 THEN 1 
                    ELSE 2 
                END,
                created_at DESC
        "#;

        let search_pattern = format!("%{}%", query_text);
        let problems = sqlx::query_as::<_, Problem>(query)
            .bind(&search_pattern)
            .fetch_all(&self.pool)
            .await?;

        Ok(problems)
    }

    async fn count(&self) -> Result<i64> {
        let query = "SELECT COUNT(*) as count FROM problems";

        let row: (i64,) = sqlx::query_as(query).fetch_one(&self.pool).await?;

        Ok(row.0)
    }
}
