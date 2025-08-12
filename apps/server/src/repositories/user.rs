use async_trait::async_trait;
use sqlx::PgPool;

use uuid::Uuid;

use crate::errors::{RepositoryError, Result};
use crate::models::user::*;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create(&self, user: CreateUserRequest) -> Result<User>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn update(&self, id: Uuid, update: UpdateUserRequest) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn list(&self, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<User>>;
    async fn count(&self) -> Result<i64>;
}

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create(&self, user: CreateUserRequest) -> Result<User> {
        let query = r#"
            INSERT INTO users (username, email, password_hash, role)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, email, password_hash, avatar_url, role, created_at, updated_at
        "#;

        let user = sqlx::query_as::<_, User>(query)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(&user.role)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let query = "SELECT * FROM users WHERE id = $1";

        let user = sqlx::query_as::<_, User>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let query = "SELECT * FROM users WHERE email = $1";

        let user = sqlx::query_as::<_, User>(query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let query = "SELECT * FROM users WHERE username = $1";

        let user = sqlx::query_as::<_, User>(query)
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(user)
    }

    async fn update(&self, id: Uuid, update: UpdateUserRequest) -> Result<User> {
        let query = r#"
            UPDATE users 
            SET username = COALESCE($2, username),
                email = COALESCE($3, email),
                avatar_url = COALESCE($4, avatar_url),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
        "#;

        let user = sqlx::query_as::<_, User>(query)
            .bind(id)
            .bind(&update.username)
            .bind(&update.email)
            .bind(&update.avatar_url)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let query = "DELETE FROM users WHERE id = $1";

        sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn list(&self, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<User>> {
        let query = r#"
            SELECT * FROM users 
            ORDER BY created_at DESC 
            LIMIT $1 OFFSET $2
        "#;

        let users = sqlx::query_as::<_, User>(query)
            .bind(limit.unwrap_or(50))
            .bind(offset.unwrap_or(0))
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(users)
    }

    async fn count(&self) -> Result<i64> {
        let query = "SELECT COUNT(*) as count FROM users";
        let row: (i64,) = sqlx::query_as(query).fetch_one(&self.pool).await?;
        Ok(row.0)
    }
}
