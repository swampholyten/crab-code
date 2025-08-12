use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    errors::Result,
    models::{problem::Problem, tag::*},
};

#[async_trait]
pub trait TagRepositoryTrait: Send + Sync {
    async fn create(&self, tag: CreateTagRequest) -> Result<Tag>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Tag>>;
    async fn list(&self) -> Result<Vec<Tag>>;
    async fn delete(&self, name: &str) -> Result<()>;
    async fn exists(&self, name: &str) -> Result<bool>;

    // Problem-Tag relationship operations
    async fn find_problems_by_tag(&self, tag_name: &str) -> Result<Vec<Problem>>;
    async fn add_tag_to_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()>;
    async fn remove_tag_from_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()>;
    async fn find_tags_for_problem(&self, problem_id: Uuid) -> Result<Vec<Tag>>;
    async fn is_tag_assigned_to_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<bool>;

    // Statistics and popular tags
    async fn get_popular_tags(&self, limit: Option<i32>) -> Result<Vec<TagWithCount>>;
    async fn count_problems_for_tag(&self, tag_name: &str) -> Result<i64>;
}

pub struct TagRepository {
    pool: PgPool,
}

impl TagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagRepositoryTrait for TagRepository {
    async fn create(&self, tag: CreateTagRequest) -> Result<Tag> {
        let query = r#"
            INSERT INTO tags (name, description)
            VALUES ($1, $2)
            RETURNING name, description, created_at
        "#;

        let tag = sqlx::query_as::<_, Tag>(query)
            .bind(&tag.name)
            .bind(&tag.description)
            .fetch_one(&self.pool)
            .await?;

        Ok(tag)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Tag>> {
        let query = "SELECT * FROM tags WHERE name = $1";

        let tag = sqlx::query_as::<_, Tag>(query)
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(tag)
    }

    async fn list(&self) -> Result<Vec<Tag>> {
        let query = "SELECT * FROM tags ORDER BY name ASC";

        let tags = sqlx::query_as::<_, Tag>(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(tags)
    }

    async fn delete(&self, name: &str) -> Result<()> {
        let query = "DELETE FROM tags WHERE name = $1";

        sqlx::query(query).bind(name).execute(&self.pool).await?;

        Ok(())
    }

    async fn exists(&self, name: &str) -> Result<bool> {
        let query = "SELECT EXISTS(SELECT 1 FROM tags WHERE name = $1)";

        let exists: (bool,) = sqlx::query_as(query)
            .bind(name)
            .fetch_one(&self.pool)
            .await?;

        Ok(exists.0)
    }

    async fn find_problems_by_tag(&self, tag_name: &str) -> Result<Vec<Problem>> {
        let query = r#"
            SELECT p.* FROM problems p
            INNER JOIN problem_tags pt ON p.id = pt.problem_id
            WHERE pt.tag_id = $1
            ORDER BY p.created_at DESC
        "#;

        let problems = sqlx::query_as::<_, Problem>(query)
            .bind(tag_name)
            .fetch_all(&self.pool)
            .await?;

        Ok(problems)
    }

    async fn add_tag_to_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()> {
        let query = r#"
            INSERT INTO problem_tags (problem_id, tag_id)
            VALUES ($1, $2)
            ON CONFLICT (problem_id, tag_id) DO NOTHING
        "#;

        sqlx::query(query)
            .bind(problem_id)
            .bind(tag_name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn remove_tag_from_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()> {
        let query = "DELETE FROM problem_tags WHERE problem_id = $1 AND tag_id = $2";

        sqlx::query(query)
            .bind(problem_id)
            .bind(tag_name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_tags_for_problem(&self, problem_id: Uuid) -> Result<Vec<Tag>> {
        let query = r#"
            SELECT t.* FROM tags t
            INNER JOIN problem_tags pt ON t.name = pt.tag_id
            WHERE pt.problem_id = $1
            ORDER BY t.name ASC
        "#;

        let tags = sqlx::query_as::<_, Tag>(query)
            .bind(problem_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(tags)
    }

    async fn is_tag_assigned_to_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<bool> {
        let query = r#"
            SELECT EXISTS(
                SELECT 1 FROM problem_tags 
                WHERE problem_id = $1 AND tag_id = $2
            )
        "#;

        let exists: (bool,) = sqlx::query_as(query)
            .bind(problem_id)
            .bind(tag_name)
            .fetch_one(&self.pool)
            .await?;

        Ok(exists.0)
    }

    async fn get_popular_tags(&self, limit: Option<i32>) -> Result<Vec<TagWithCount>> {
        let mut query = String::from(
            r#"
            SELECT t.name, t.description, COUNT(pt.problem_id) as problem_count
            FROM tags t
            LEFT JOIN problem_tags pt ON t.name = pt.tag_id
            GROUP BY t.name, t.description
            ORDER BY problem_count DESC, t.name ASC
        "#,
        );

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        let rows = sqlx::query(&query).fetch_all(&self.pool).await?;

        let mut tags_with_count = Vec::new();
        for row in rows {
            let tag_with_count = TagWithCount {
                name: row.get("name"),
                description: row.get("description"),
                problem_count: row.get("problem_count"),
            };
            tags_with_count.push(tag_with_count);
        }

        Ok(tags_with_count)
    }

    async fn count_problems_for_tag(&self, tag_name: &str) -> Result<i64> {
        let query = r#"
            SELECT COUNT(*) as count FROM problem_tags 
            WHERE tag_id = $1
        "#;

        let row: (i64,) = sqlx::query_as(query)
            .bind(tag_name)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }
}
