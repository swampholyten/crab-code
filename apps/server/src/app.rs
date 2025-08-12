use crate::{common::state::AppState, errors::Result};
use axum::{Router, http::StatusCode, response::IntoResponse};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::common::config;

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
