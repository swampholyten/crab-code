use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{common::state::AppState, handlers::user};

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(user::create_user))
        .route("/", get(user::list_users))
        .route("/{id}", get(user::get_user_by_id))
        .route("/{id}", patch(user::update_user))
        .route("/{id}", delete(user::delete_user))
        .route("/{id}/profile", get(user::get_user_profile))
}

pub fn router() -> Router<AppState> {
    Router::new().nest("/users", user_routes())
}
