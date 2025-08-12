use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

use crate::{
    common::state::AppState,
    handlers::{problem, submission, tag, test_case},
};

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
        .route("/{problem_id}/tags", get(tag::get_tags_for_problem))
        .route(
            "/{problem_id}/tags/{tag_name}",
            post(tag::add_tag_to_problem),
        )
        .route(
            "/{problem_id}/tags/{tag_name}",
            delete(tag::remove_tag_from_problem),
        )
        // Bulk operations
        .route(
            "/{problem_id}/tags/bulk",
            post(tag::bulk_add_tags_to_problem),
        )
        .route("/{problem_id}/tags/replace", put(tag::replace_problem_tags))
        .route(
            "/{problem_id}/submissions",
            get(submission::get_problem_submissions),
        )
        .route(
            "/{problem_id}/submissions/stats",
            get(submission::get_problem_submission_stats),
        )
        .route(
            "/{problem_id}/test-cases",
            post(test_case::create_test_case),
        )
        .route(
            "/{problem_id}/test-cases",
            get(test_case::get_test_cases_for_problem),
        )
        .route(
            "/{problem_id}/test-cases/sample",
            get(test_case::get_sample_test_cases),
        )
        .route(
            "/{problem_id}/test-cases/bulk",
            post(test_case::bulk_create_test_cases),
        )
}

pub fn router() -> Router<AppState> {
    Router::new().nest("/problems", problem_routes())
}
