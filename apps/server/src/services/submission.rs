use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::Result,
    models::submission::{CreateSubmissionRequest, JudgeResult, Submission, SubmissionFilter},
};

#[async_trait]
pub trait SubmissionServiceTrait: Send + Sync {
    async fn create_submission(&self, request: CreateSubmissionRequest) -> Result<Submission>;
    async fn get_submission_by_id(&self, id: Uuid) -> Result<Option<Submission>>;
    async fn get_user_submissions(
        &self,
        user_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>>;
    async fn get_problem_submissions(
        &self,
        problem_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>>;
    async fn update_submission_result(&self, id: Uuid, result: JudgeResult) -> Result<()>;
}
