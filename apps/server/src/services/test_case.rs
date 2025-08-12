use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::{Result, ServiceError},
    models::test_case::{CreateTestCaseRequest, TestCase, UpdateTestCaseRequest},
    repositories::{problem::ProblemRepositoryTrait, test_case::TestCaseRepositoryTrait},
};

#[async_trait]
pub trait TestCaseServiceTrait: Send + Sync {
    async fn create_test_case(&self, request: CreateTestCaseRequest) -> Result<TestCase>;
    async fn get_test_case_by_id(&self, id: Uuid) -> Result<Option<TestCase>>;
    async fn get_test_cases_for_problem(
        &self,
        problem_id: Uuid,
        include_hidden: bool,
    ) -> Result<Vec<TestCase>>;
    async fn get_sample_test_cases(&self, problem_id: Uuid) -> Result<Vec<TestCase>>;
    async fn get_hidden_test_cases(&self, problem_id: Uuid) -> Result<Vec<TestCase>>;
    async fn update_test_case(&self, id: Uuid, update: UpdateTestCaseRequest) -> Result<TestCase>;
    async fn delete_test_case(&self, id: Uuid) -> Result<()>;
    async fn bulk_create_test_cases(
        &self,
        test_cases: Vec<CreateTestCaseRequest>,
    ) -> Result<Vec<TestCase>>;

    async fn validate_test_case_format(
        &self,
        input_data: &str,
        expected_output: &str,
    ) -> Result<()>;
}

#[derive(Clone)]
pub struct TestCaseService {
    test_case_repository: Arc<dyn TestCaseRepositoryTrait + Send + Sync>,
    problem_repository: Arc<dyn ProblemRepositoryTrait + Send + Sync>,
}

impl TestCaseService {
    pub fn new(
        test_case_repository: Arc<dyn TestCaseRepositoryTrait + Send + Sync>,
        problem_repository: Arc<dyn ProblemRepositoryTrait + Send + Sync>,
    ) -> Self {
        Self {
            test_case_repository,
            problem_repository,
        }
    }
}

#[async_trait]
impl TestCaseServiceTrait for TestCaseService {
    async fn create_test_case(&self, request: CreateTestCaseRequest) -> Result<TestCase> {
        if self
            .problem_repository
            .find_by_id(request.problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        self.validate_test_case_format(&request.input_data, &request.expected_output)
            .await?;

        let test_case = self.test_case_repository.create(request).await?;
        Ok(test_case)
    }

    async fn get_test_case_by_id(&self, id: Uuid) -> Result<Option<TestCase>> {
        self.test_case_repository.find_by_id(id).await
    }

    async fn get_test_cases_for_problem(
        &self,
        problem_id: Uuid,
        include_hidden: bool,
    ) -> Result<Vec<TestCase>> {
        // Validate problem exists
        if self
            .problem_repository
            .find_by_id(problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        self.test_case_repository
            .find_by_problem(problem_id, include_hidden)
            .await
    }

    async fn get_sample_test_cases(&self, problem_id: Uuid) -> Result<Vec<TestCase>> {
        if self
            .problem_repository
            .find_by_id(problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        self.test_case_repository
            .find_sample_cases(problem_id)
            .await
    }

    async fn get_hidden_test_cases(&self, problem_id: Uuid) -> Result<Vec<TestCase>> {
        if self
            .problem_repository
            .find_by_id(problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        self.test_case_repository
            .find_hidden_cases(problem_id)
            .await
    }

    async fn update_test_case(&self, id: Uuid, update: UpdateTestCaseRequest) -> Result<TestCase> {
        if self.test_case_repository.find_by_id(id).await?.is_none() {
            return Err(ServiceError::NotFoundError("Test case not found".to_string()).into());
        }

        // Validate format if data is being updated
        if let (Some(input), Some(output)) = (&update.input_data, &update.expected_output) {
            self.validate_test_case_format(input, output).await?;
        }

        self.test_case_repository.update(id, update).await
    }

    async fn delete_test_case(&self, id: Uuid) -> Result<()> {
        // Check if test case exists
        if self.test_case_repository.find_by_id(id).await?.is_none() {
            return Err(ServiceError::NotFoundError("Test case not found".to_string()).into());
        }

        self.test_case_repository.delete(id).await
    }

    async fn bulk_create_test_cases(
        &self,
        test_cases: Vec<CreateTestCaseRequest>,
    ) -> Result<Vec<TestCase>> {
        if test_cases.is_empty() {
            return Err(ServiceError::ValidationError("No test cases provided".to_string()).into());
        }

        // Validate all test cases belong to the same problem
        let first_problem_id = test_cases[0].problem_id;
        if !test_cases
            .iter()
            .all(|tc| tc.problem_id == first_problem_id)
        {
            return Err(ServiceError::ValidationError(
                "All test cases must belong to the same problem".to_string(),
            )
            .into());
        }

        // Validate problem exists
        if self
            .problem_repository
            .find_by_id(first_problem_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFoundError("Problem not found".to_string()).into());
        }

        // Validate all test cases format
        for test_case in &test_cases {
            self.validate_test_case_format(&test_case.input_data, &test_case.expected_output)
                .await?;
        }

        self.test_case_repository.bulk_create(test_cases).await
    }

    async fn validate_test_case_format(
        &self,
        input_data: &str,
        expected_output: &str,
    ) -> Result<()> {
        if input_data.trim().is_empty() {
            return Err(ServiceError::ValidationError("Input data cannot be empty".into()).into());
        }

        if expected_output.trim().is_empty() {
            return Err(
                ServiceError::ValidationError("Expected output cannot be empty".into()).into(),
            );
        }

        const MAX_SIZE: usize = 1024 * 1024;

        if input_data.len() > MAX_SIZE {
            return Err(
                ServiceError::ValidationError("Input data too large (max 1MB)".into()).into(),
            );
        }

        if expected_output.len() > MAX_SIZE {
            return Err(ServiceError::ValidationError(
                "Expected output too large (max 1MB)".to_string(),
            )
            .into());
        }

        // Additional format validation can be added here
        // e.g., JSON validation, numeric validation, etc.

        Ok(())
    }
}
