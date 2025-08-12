use axum::{
    Router,
    routing::{get, patch},
};

use crate::{common::state::AppState, handlers::submission};

fn submission_routes() -> Router<AppState> {
    Router::new()
        .route("/{submission_id}", get(submission::get_submission_by_id))
        .route(
            "/{submission_id}/result",
            patch(submission::update_submission_result),
        )
        .route(
            "/{submission_id}/code",
            get(submission::get_submission_code),
        )
}

pub fn router() -> Router<AppState> {
    Router::new().nest("/submissions", submission_routes())
}
