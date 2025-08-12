use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{
    common::state::AppState,
    handlers::{submission, user},
};

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(user::create_user))
        .route("/", get(user::list_users))
        .route("/{id}", get(user::get_user_by_id))
        .route("/{id}", patch(user::update_user))
        .route("/{id}", delete(user::delete_user))
        .route("/{id}/profile", get(user::get_user_profile))
        .route(
            "/{user_id}/submissions",
            get(submission::get_user_submissions),
        )
        .route(
            "/{user_id}/submissions/stats",
            get(submission::get_user_submission_stats),
        )
        .route(
            "/{user_id}/submissions",
            post(submission::create_submission),
        )
}

pub fn router() -> Router<AppState> {
    Router::new().nest("/users", user_routes())
}
