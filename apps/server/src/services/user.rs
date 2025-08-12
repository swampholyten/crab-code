use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::Result,
    models::user::{CreateUserRequest, UpdateUserRequest, User, UserProfile},
};

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn update_user(&self, id: Uuid, update: UpdateUserRequest) -> Result<User>;
    async fn delete_user(&self, id: Uuid) -> Result<()>;
    async fn get_user_profile(&self, id: Uuid) -> Result<UserProfile>;
}
