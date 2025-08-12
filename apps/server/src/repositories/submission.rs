use uuid::Uuid;

use crate::{errors::Result, models::submission::*};

pub trait SubmissionRepositoryTrait: Send + Sync {
    async fn create(&self, submission: CreateSubmissionRequest) -> Result<Submission>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Submission>>;
    async fn find_by_user(
        &self,
        user_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>>;
    async fn find_by_problem(
        &self,
        problem_id: Uuid,
        filter: SubmissionFilter,
    ) -> Result<Vec<Submission>>;
    async fn update_status(
        &self,
        id: Uuid,
        status: SubmissionStatus,
        execution_time: Option<i32>,
        memory_used: Option<i32>,
        error_message: Option<String>,
    ) -> Result<()>;
    async fn find_latest_accepted(
        &self,
        user_id: Uuid,
        problem_id: Uuid,
    ) -> Result<Option<Submission>>;
    async fn count_by_status(&self, status: SubmissionStatus) -> Result<i64>;
}
