use uuid::Uuid;

use crate::{errors::Result, models::problem::*};

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
