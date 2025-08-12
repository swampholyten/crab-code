use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{common::state::AppState, handlers::language};

fn language_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(language::create_language))
        .route("/", get(language::list_languages))
        .route("/{name}", get(language::get_language_by_name))
        .route("/{name}", patch(language::update_language))
        .route("/{name}", delete(language::delete_language))
        .route("/{name}/supported", get(language::check_language_support))
        .route("/supported/list", get(language::get_supported_languages))
}

pub fn router() -> Router<AppState> {
    Router::new().nest("/languages", language_routes())
}
