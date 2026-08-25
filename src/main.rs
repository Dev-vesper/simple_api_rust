mod db;
mod handlers;
mod models;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    db::init_db()?;

    let app = Router::new()
        .route("/users", get(handlers::list_users).post(handlers::create_user))
        .route("/users/sorted", get(handlers::sorted_users))
        .route(
            "/users/:id",
            put(handlers::update_user).delete(handlers::delete_user),
        );

    let addr = "0.0.0.0:5070";
    tracing::info!("Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
