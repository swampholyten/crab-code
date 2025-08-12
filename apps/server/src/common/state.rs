use std::sync::Arc;

use crate::services::{
    auth::AuthServiceTrait, judge::JudgeServiceTrait, language::LanguageServiceTrait,
    problem::ProblemServiceTrait, stats::StatsServiceTrait, submission::SubmissionServiceTrait,
    tag::TagServiceTrait, test_case::TestCaseServiceTrait, user::UserServiceTrait,
};

use super::config::Config;

#[derive(Clone)]
pub struct AppState {
    /// Global application configuration.
    pub config: Config,
    // Service handling authentication-related logic.
    // pub auth_service: Arc<dyn AuthServiceTrait>,
    /// Service handling user-related logic.
    pub user_service: Arc<dyn UserServiceTrait>,
    /// Service handling problem-related logic.
    pub problem_service: Arc<dyn ProblemServiceTrait>,
    /// Service handling code submission and execution.
    pub submission_service: Arc<dyn SubmissionServiceTrait>,
    // Service handling code execution and judging.
    // pub judge_service: Arc<dyn JudgeServiceTrait>,
    /// Service handling programming languages.
    pub language_service: Arc<dyn LanguageServiceTrait>,
    /// Service handling tags and categorization.
    pub tag_service: Arc<dyn TagServiceTrait>,
    /// Service handling test cases for problems.
    pub test_case_service: Arc<dyn TestCaseServiceTrait>,
    // Service handling user statistics and progress.
    // pub stats_service: Arc<dyn StatsServiceTrait>,
}

impl AppState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: Config,
        // auth_service: Arc<dyn AuthServiceTrait>,
        user_service: Arc<dyn UserServiceTrait>,
        problem_service: Arc<dyn ProblemServiceTrait>,
        submission_service: Arc<dyn SubmissionServiceTrait>,
        // judge_service: Arc<dyn JudgeServiceTrait>,
        language_service: Arc<dyn LanguageServiceTrait>,
        tag_service: Arc<dyn TagServiceTrait>,
        test_case_service: Arc<dyn TestCaseServiceTrait>,
        // stats_service: Arc<dyn StatsServiceTrait>,
    ) -> Self {
        Self {
            config,
            // auth_service,
            user_service,
            problem_service,
            submission_service,
            // judge_service,
            language_service,
            tag_service,
            test_case_service,
            // stats_service,
        }
    }
}
