use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{common::state::AppState, handlers::tag};

fn tag_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(tag::create_tag))
        .route("/", get(tag::get_all_tags))
        .route("/{name}", get(tag::get_tag_by_name))
        .route("/{name}", delete(tag::delete_tag))
        .route("/{tag_name}/problems", get(tag::get_problems_by_tag))
        .route("/popular", get(tag::get_popular_tags))
}

pub fn router() -> Router<AppState> {
    Router::new().nest("/tags", tag_routes())
}
