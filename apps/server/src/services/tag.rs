use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::Result,
    models::{
        problem::Problem,
        tag::{CreateTagRequest, Tag, TagWithCount},
    },
};

#[async_trait]
pub trait TagServiceTrait: Send + Sync {
    async fn get_all_tags(&self) -> Result<Vec<Tag>>;
    async fn create_tag(&self, request: CreateTagRequest) -> Result<Tag>;
    async fn get_problems_by_tag(&self, tag_name: &str) -> Result<Vec<Problem>>;
    async fn add_tag_to_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()>;
    async fn remove_tag_from_problem(&self, problem_id: Uuid, tag_name: &str) -> Result<()>;
    async fn get_popular_tags(&self, limit: Option<i32>) -> Result<Vec<TagWithCount>>;
}
