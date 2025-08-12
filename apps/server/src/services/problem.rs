use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::Result,
    models::problem::{
        CreateProblemRequest, Problem, ProblemFilter, ProblemWithTags, UpdateProblemRequest,
    },
};

#[async_trait]
pub trait ProblemServiceTrait: Send + Sync {
    async fn create_problem(&self, request: CreateProblemRequest) -> Result<Problem>;
    async fn get_problem_by_id(&self, id: Uuid) -> Result<Option<Problem>>;
    async fn get_problem_by_slug(&self, slug: &str) -> Result<Option<Problem>>;
    async fn list_problems(&self, filter: ProblemFilter) -> Result<Vec<Problem>>;
    async fn update_problem(&self, id: Uuid, update: UpdateProblemRequest) -> Result<Problem>;
    async fn delete_problem(&self, id: Uuid) -> Result<()>;
    async fn search_problems(&self, query: &str) -> Result<Vec<Problem>>;
    async fn get_problem_with_tags(&self, id: Uuid) -> Result<Option<ProblemWithTags>>;
}
