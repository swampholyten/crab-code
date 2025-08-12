use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::Result,
    models::{
        submission::{JudgeResult, TestResult, ValidationResult},
        test_case::TestCase,
    },
};

#[async_trait]
pub trait JudgeServiceTrait: Send + Sync {
    async fn execute_submission(&self, submission_id: Uuid) -> Result<JudgeResult>;
    async fn validate_code(&self, code: &str, language: &str) -> Result<ValidationResult>;
    async fn run_test_cases(
        &self,
        code: &str,
        language: &str,
        test_cases: Vec<TestCase>,
    ) -> Result<Vec<TestResult>>;
}
