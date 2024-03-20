mod config;
mod error;
mod log;
mod service;
mod api;

pub use self::{config::Config, error::Error, log::log_request};

use axum::{middleware, Router};
use base64::engine::general_purpose;
use base64::Engine;
use sqlx::postgres::PgPoolOptions;
use tokio::{signal, task::AbortHandle};
use tower_cookies::{CookieManagerLayer, Key};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{
    service::{auth::{auth, session}, email, room},
    api::{
        auth::{
            mw_auth::{mw_ctx_resolver, mw_require_auth},
            SESSION_COOKIE_KEY,
        },
        response_mapper::main_response_mapper,
    },
};

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    // logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let config = Config::load_from_env()?;
    let config = dbg!(config);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&config.database_uri)
        .await
        .unwrap();

    tracing::info!("connected to database");

    // services
    let email_service = email::Service::new(email::Config::from(config.clone()))
        .await
        .map_err(|e| Error::InvalidConfig {
            detail: e.to_string(),
        })?;
    let _auth_service = auth::Service::new(pool.clone(), email_service);
    let session_service = session::Service::new(pool.clone());
    let room_service = room::Service::new(pool.clone());

    let deletion_task = tokio::task::spawn(
        session_service
            .clone()
            .continously_delete_expired_sessions(tokio::time::Duration::from_secs(60)),
    );

    let key = general_purpose::STANDARD
        .decode(config.session_key)
        .unwrap();
    SESSION_COOKIE_KEY
        .set(Key::from(key.as_ref()))
        .expect("cannot set key");

    let _app_routes = api::app::index::routes(room_service);
    let app = Router::new()
        .layer(middleware::from_fn(mw_require_auth))
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            session_service,
            mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind(config.listen_address)
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await
        .unwrap();
    Ok(())
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
