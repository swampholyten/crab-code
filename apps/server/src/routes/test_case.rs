use axum::{
    Router,
    routing::{delete, get, patch},
};

use crate::{common::state::AppState, handlers::test_case};

pub fn test_case_routes() -> Router<AppState> {
    Router::new()
        .route("/{test_case_id}", get(test_case::get_test_case_by_id))
        .route("/{test_case_id}", patch(test_case::update_test_case))
        .route("/{test_case_id}", delete(test_case::delete_test_case))
}

pub fn router() -> Router<AppState> {
    Router::new().nest("/test_cases", test_case_routes())
}
