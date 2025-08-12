use uuid::Uuid;

use crate::domains::user::domain::model::{CreateUserRequest, UpdateUserRequest, User};

pub trait UserRepositoryTrait: Send + Sync {
    async fn create(&self, user: CreateUserRequest) -> Result<User>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn update(&self, id: Uuid, update: UpdateUserRequest) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn list(&self, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<User>>;
}
