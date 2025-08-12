use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::Result,
    models::test_case::{CreateTestCaseRequest, TestCase, UpdateTestCaseRequest},
};

#[async_trait]
pub trait TestCaseServiceTrait: Send + Sync {
    async fn create_test_case(&self, request: CreateTestCaseRequest) -> Result<TestCase>;
    async fn get_test_cases_for_problem(
        &self,
        problem_id: Uuid,
        include_hidden: bool,
    ) -> Result<Vec<TestCase>>;
    async fn get_sample_test_cases(&self, problem_id: Uuid) -> Result<Vec<TestCase>>;
    async fn update_test_case(&self, id: Uuid, update: UpdateTestCaseRequest) -> Result<TestCase>;
    async fn delete_test_case(&self, id: Uuid) -> Result<()>;
}
