use simple_api_rust::{db::Database, handlers};

use axum::{
    routing::{get, put},
    Router,
};
use std::path::Path;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Use the default database path (or from env for production flexibility).
    let db_path = std::env::var("DB_PATH").unwrap_or_else(|_| "data/app.db".to_string());
    let database = Database::new(Path::new(&db_path))?;

    let app = Router::new()
        .route("/users", get(handlers::list_users).post(handlers::create_user))
        .route("/users/sorted", get(handlers::sorted_users))
        .route(
            "/users/{id}",
            put(handlers::update_user).delete(handlers::delete_user),
        )
        .with_state(database); // Pass the Database as shared state

    let addr = "0.0.0.0:5070";
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
