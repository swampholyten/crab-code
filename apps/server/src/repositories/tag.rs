use async_trait::async_trait;
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
    async fn find_problems_by_tag(&self, tag_name: &str) -> Result<Vec<Problem>>;
    async fn add_tag_to_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()>;
    async fn remove_tag_from_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()>;
    async fn find_tags_for_problem(&self, problem_id: Uuid) -> Result<Vec<Tag>>;
    async fn get_popular_tags(&self, limit: Option<i32>) -> Result<Vec<TagWithCount>>;
}
