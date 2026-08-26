use axum::{
    extract::DefaultBodyLimit,
    routing::{get, put},
    Router,
};

use crate::{db::Database, handlers};

pub const MAX_BODY_BYTES: usize = 16 * 1024;

pub fn build_router(database: Database) -> Router {
    Router::new()
        .route("/users", get(handlers::list_users).post(handlers::create_user))
        .route("/users/sorted", get(handlers::sorted_users))
        .route(
            "/users/{id}",
            put(handlers::update_user).delete(handlers::delete_user),
        )
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        .with_state(database)
}
