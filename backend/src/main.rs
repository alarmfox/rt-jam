mod log;
mod service;
mod web;

use axum::{middleware, routing::get, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::EnvFilter;
use web::{
    mw_auth::{mw_ctx_require, mw_ctx_resolver}, mw_req_stamp::mw_req_stamp_resolver, mw_res_map::mw_reponse_map
};
use service::user::session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init logging
    tracing_subscriber::fmt()
        .with_target(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db =
        connect_to_db("postgres://postgres:postgres@localhost/postgres?sslmode=disable").await?;

    let app = Router::new().route("/test", get(|| async { "Hello world" }));

    let app = app
        .layer(middleware::from_fn(mw_ctx_require))
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(
            session::Service::new(db),
            mw_ctx_resolver,
        ))
        .layer(middleware::from_fn(mw_req_stamp_resolver))
        .layer(CookieManagerLayer::new());

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
