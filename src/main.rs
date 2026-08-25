mod db;
mod handlers;
mod models;

use axum::{
    routing::{get, put},
    Router,
};
use tracing_subscriber;

#[tokio::main] // Sets up the Tokio async runtime and makes `main` async
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Create the database table if it doesn't exist.
    db::init_db()?;

    // Build the service router.
    // Each route maps an HTTP method and path to a handler function.
    let app = Router::new()
        .route("/users", get(handlers::list_users).post(handlers::create_user))
        .route("/users/sorted", get(handlers::sorted_users))
        .route(
            "/users/:id",
            put(handlers::update_user).delete(handlers::delete_user),
        );

    // Bind to all network interfaces on port 5070.
    // This matches the port exposed in the Dockerfile.
    let addr = "0.0.0.0:5070";
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    // Start serving requests. This call runs forever until the process is killed.
    axum::serve(listener, app).await?;

    Ok(())
}
