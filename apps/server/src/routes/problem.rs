use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{common::state::AppState, handlers::problem};

fn problem_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(problem::create_problem))
        .route("/", get(problem::list_problems))
        .route("/search", get(problem::search_problems))
        .route("/{id}", get(problem::get_problem_by_id))
        .route("/{id}", patch(problem::update_problem))
        .route("/{id}", delete(problem::delete_problem))
        .route("/slug/{slug}", get(problem::get_problem_by_slug))
        .route(
            "/difficulty/{difficulty}",
            get(problem::get_problems_by_difficulty),
        )
}

pub fn router() -> Router<AppState> {
    Router::new().nest("/problems", problem_routes())
}
