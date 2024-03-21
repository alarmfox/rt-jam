mod service;
mod log;
mod web;

use axum::{routing::get, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db = connect_to_db("postgres://postgres:postgres@localhost/postgtres?sslmode=disable").await?;

    let app = Router::new().route("/test", get(|| async { "Hello world" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

async fn connect_to_db(dsn: &'static str) -> Result<PgPool, Box<dyn std::error::Error>> {
    let conn = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(dsn)
        .await?;

    Ok(conn)
}
