use axum::{Router, http::StatusCode, response::IntoResponse};

use crate::common::error::Result;
use crate::common::state::State;

/// Create the main router
pub fn create_router(state: State) -> Router {
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
