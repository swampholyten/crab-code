use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::Result;
use crate::models::{problem::Problem, user::UserStats};

#[async_trait]
pub trait StatsServiceTrait: Send + Sync {
    async fn get_user_stats(&self, user_id: Uuid) -> Result<UserStats>;
    async fn update_user_stats(&self, user_id: Uuid) -> Result<()>;
    async fn get_user_solved_problems(&self, user_id: Uuid) -> Result<Vec<Problem>>;
}
