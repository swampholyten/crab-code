use async_trait::async_trait;
use uuid::Uuid;

use crate::{errors::Result, models::test_case::*};

#[async_trait]
pub trait TestCaseRepositoryTrait: Send + Sync {
    async fn create(&self, test_case: CreateTestCaseRequest) -> Result<TestCase>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TestCase>>;
    async fn find_by_problem(
        &self,
        problem_id: Uuid,
        include_hidden: bool,
    ) -> Result<Vec<TestCase>>;
    async fn find_sample_cases(&self, problem_id: Uuid) -> Result<Vec<TestCase>>;
    async fn update(&self, id: Uuid, update: UpdateTestCaseRequest) -> Result<TestCase>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn count_by_problem(&self, problem_id: Uuid) -> Result<i64>;
}
