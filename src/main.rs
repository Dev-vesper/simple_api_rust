use simple_api_rust::{db::Database, routes};

use std::path::Path;
// use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Use the default database path (or from env for production flexibility).
    let db_path = std::env::var("DB_PATH").unwrap_or_else(|_| "data/app.db".to_string());
    let database = Database::new(Path::new(&db_path))?;

    let app = routes::build_router(database);

    let addr = "0.0.0.0:5070";
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
