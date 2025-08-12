use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::{
        response::{ApiResponse, ApiResult},
        state::AppState,
    },
    models::test_case::TestCase,
};

#[derive(Debug, Deserialize)]
pub struct CreateTestCaseRequest {
    pub input_data: String,
    pub expected_output: String,
    pub is_sample: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTestCaseRequest {
    pub input_data: Option<String>,
    pub expected_output: Option<String>,
    pub is_sample: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct BulkCreateTestCasesRequest {
    pub test_cases: Vec<CreateTestCaseRequest>,
}

#[derive(Debug, Deserialize)]
pub struct TestCaseQuery {
    pub include_hidden: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TestCaseResponse {
    pub id: Uuid,
    pub problem_id: Uuid,
    pub input_data: String,
    pub expected_output: String,
    pub is_sample: bool,
    pub created_at: DateTime<Utc>,
}

impl From<TestCase> for TestCaseResponse {
    fn from(test_case: TestCase) -> Self {
        Self {
            id: test_case.id,
            problem_id: test_case.problem_id,
            input_data: test_case.input_data,
            expected_output: test_case.expected_output,
            is_sample: test_case.is_sample,
            created_at: test_case.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TestCaseSummaryResponse {
    pub id: Uuid,
    pub input_data: String,
    pub expected_output: String,
    pub is_sample: bool,
}

impl From<TestCase> for TestCaseSummaryResponse {
    fn from(test_case: TestCase) -> Self {
        Self {
            id: test_case.id,
            input_data: test_case.input_data,
            expected_output: test_case.expected_output,
            is_sample: test_case.is_sample,
        }
    }
}

pub async fn create_test_case(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
    Json(request): Json<CreateTestCaseRequest>,
) -> ApiResult<TestCaseResponse> {
    let create_request = crate::models::test_case::CreateTestCaseRequest {
        problem_id,
        input_data: request.input_data,
        expected_output: request.expected_output,
        is_sample: request.is_sample,
    };

    let test_case = state
        .test_case_service
        .create_test_case(create_request)
        .await?;
    let response = ApiResponse::success_with_message(
        test_case.into(),
        "Test case created successfully".to_string(),
    );

    Ok(Json(response))
}

pub async fn get_test_case_by_id(
    State(state): State<AppState>,
    Path(test_case_id): Path<Uuid>,
) -> ApiResult<TestCaseResponse> {
    let test_case = state
        .test_case_service
        .get_test_case_by_id(test_case_id)
        .await?
        .ok_or_else(|| crate::errors::Error::NotFound("Test case not found".into()))?;

    let response = ApiResponse::success(test_case.into());

    Ok(Json(response))
}

pub async fn get_test_cases_for_problem(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
    Query(query): Query<TestCaseQuery>,
) -> ApiResult<Vec<TestCaseSummaryResponse>> {
    let include_hidden = query.include_hidden.unwrap_or(false);
    let test_cases = state
        .test_case_service
        .get_test_cases_for_problem(problem_id, include_hidden)
        .await?;

    let test_case_responses: Vec<TestCaseSummaryResponse> =
        test_cases.into_iter().map(|tc| tc.into()).collect();

    let response = ApiResponse::success(test_case_responses);
    Ok(Json(response))
}

pub async fn get_sample_test_cases(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
) -> ApiResult<Vec<TestCaseSummaryResponse>> {
    let test_cases = state
        .test_case_service
        .get_sample_test_cases(problem_id)
        .await?;

    let test_case_responses: Vec<TestCaseSummaryResponse> =
        test_cases.into_iter().map(|tc| tc.into()).collect();

    let response = ApiResponse::success(test_case_responses);
    Ok(Json(response))
}

pub async fn update_test_case(
    State(state): State<AppState>,
    Path(test_case_id): Path<Uuid>,
    Json(request): Json<UpdateTestCaseRequest>,
) -> ApiResult<TestCaseResponse> {
    let update_request = crate::models::test_case::UpdateTestCaseRequest {
        input_data: request.input_data,
        expected_output: request.expected_output,
        is_sample: request.is_sample,
    };

    let test_case = state
        .test_case_service
        .update_test_case(test_case_id, update_request)
        .await?;
    let response = ApiResponse::success_with_message(
        test_case.into(),
        "Test case updated successfully".to_string(),
    );

    Ok(Json(response))
}

pub async fn delete_test_case(
    State(state): State<AppState>,
    Path(test_case_id): Path<Uuid>,
) -> ApiResult<()> {
    state
        .test_case_service
        .delete_test_case(test_case_id)
        .await?;
    let response = ApiResponse::success_message("Test case deleted successfully".to_string());
    Ok(Json(response))
}

pub async fn bulk_create_test_cases(
    State(state): State<AppState>,
    Path(problem_id): Path<Uuid>,
    Json(request): Json<BulkCreateTestCasesRequest>,
) -> ApiResult<Vec<TestCaseResponse>> {
    let create_requests: Vec<crate::models::test_case::CreateTestCaseRequest> = request
        .test_cases
        .into_iter()
        .map(|tc| crate::models::test_case::CreateTestCaseRequest {
            problem_id,
            input_data: tc.input_data,
            expected_output: tc.expected_output,
            is_sample: tc.is_sample,
        })
        .collect();

    let test_cases = state
        .test_case_service
        .bulk_create_test_cases(create_requests)
        .await?;

    let test_case_responses: Vec<TestCaseResponse> =
        test_cases.into_iter().map(|tc| tc.into()).collect();

    let total_created = test_case_responses.len();

    let response = ApiResponse::success_with_message(
        test_case_responses,
        format!("Successfully created {} test cases", total_created),
    );

    Ok(Json(response))
}
