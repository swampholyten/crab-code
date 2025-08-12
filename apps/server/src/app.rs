use crate::{
    common::{config::Config, state::AppState},
    errors::Result,
};
use axum::{Router, http::StatusCode, response::IntoResponse};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run() {
    todo!()
    // Load configuration.
    // let config = config::load();
}

/// Create the main router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", axum::routing::get(health_check))
        .fallback(fallback)
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}

pub async fn fallback() -> Result<impl IntoResponse> {
    Ok((StatusCode::NOT_FOUND, "Not Found"))
}

pub fn setup_tracing() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE),
        )
        .init();
}

pub async fn setup_database(config: &Config) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.postgres_connection_pool)
        .connect(&config.postgres_url())
        .await?;

    Ok(pool)
}

// pub fn setup_app_state(pool: PgPool, config: Config) -> Result<AppState> {
//     let pool = Arc::new(pool);
//
//     // Create repositories
//     let user_repository = Arc::new(UserRepository::new(pool.as_ref().clone()));
//     let problem_repository = Arc::new(ProblemRepository::new(pool.as_ref().clone()));
//     let submission_repository = Arc::new(SubmissionRepository::new(pool.as_ref().clone()));
//     let language_repository = Arc::new(LanguageRepository::new(pool.as_ref().clone()));
//     let tag_repository = Arc::new(TagRepository::new(pool.as_ref().clone()));
//     let test_case_repository = Arc::new(TestCaseRepository::new(pool.as_ref().clone()));
//
//     // Create services with repository dependencies
//     let user_service = Arc::new(UserService::new(
//         user_repository.clone(),
//         submission_repository.clone(),
//     ));
//
//     let problem_service = Arc::new(ProblemService::new(
//         problem_repository.clone(),
//         tag_repository.clone(),
//         test_case_repository.clone(),
//     ));
//
//     let submission_service = Arc::new(SubmissionService::new(
//         submission_repository.clone(),
//         problem_repository.clone(),
//         user_repository.clone(),
//     ));
//
//     let judge_service = Arc::new(JudgeService::new(
//         submission_repository.clone(),
//         test_case_repository.clone(),
//         language_repository.clone(),
//     ));
//
//     let language_service = Arc::new(LanguageService::new(language_repository.clone()));
//
//     let tag_service = Arc::new(TagService::new(
//         tag_repository.clone(),
//         problem_repository.clone(),
//     ));
//
//     let test_case_service = Arc::new(TestCaseService::new(
//         test_case_repository.clone(),
//         problem_repository.clone(),
//     ));
//
//     let stats_service = Arc::new(StatsService::new(
//         submission_repository.clone(),
//         problem_repository.clone(),
//         user_repository.clone(),
//     ));
//
//     let auth_service = Arc::new(AuthService::new(
//         user_service.clone(),
//         config.jwt_secret.clone(),
//     ));
//
//     Ok(AppState::new(
//         config,
//         auth_service,
//         user_service,
//         problem_service,
//         submission_service,
//         judge_service,
//         language_service,
//         tag_service,
//         test_case_service,
//         stats_service,
//     ))
// }
