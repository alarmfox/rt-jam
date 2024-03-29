mod log;
mod service;
mod web;

use std::{future::IntoFuture, net::ToSocketAddrs};

use axum::{middleware, routing::get, Router};
use base64::{engine::general_purpose, Engine};
use service::{
    email,
    user::{auth, session},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::{signal, task::AbortHandle};
use tower_cookies::{CookieManagerLayer, Key};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use web::{
    mw_auth::{mw_ctx_require, mw_ctx_resolver},
    mw_req_stamp::mw_req_stamp_resolver,
    mw_res_map::mw_reponse_map,
    routes_login,
};

use crate::web::{
    webtransport::{self, Certs},
    SESSION_COOKIE_KEY,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init logging
    tracing_subscriber::fmt()
        .with_target(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db =
        connect_to_db("postgres://postgres:postgres@localhost/postgres?sslmode=disable").await?;

    let email_service = email::Service::new(email::Config {
        smtp_host: "mail.privateemail.com".into(),
        smtp_port: 587,
        smtp_user: "info@capass.org".into(),
        smtp_pass: "@info-Alarmfox97".into(),
        smtp_from: "info@capass.org".into(),
        app_url: "http://localhost:3000".into(),
    })
    .await?;

    let auth_service = auth::Service::new(db.clone(), email_service);
    let session_service = session::Service::new(db.clone());

    let key = general_purpose::STANDARD
        .decode("mN1GR7dsQ+Bj8NFIA+n/uvSbBcdyvHnVdFuJSJrQJ3g2/8gGYaATt3Wv7j3xKpD07652no/eddRdD7sJTVjg4w==")
        .unwrap();
    SESSION_COOKIE_KEY
        .set(Key::from(key.as_ref()))
        .expect("cannot set key");

    let deletion_task = tokio::spawn(
        session_service
            .clone()
            .continously_delete_expired_sessions(tokio::time::Duration::from_secs(60)),
    );

    let opt = webtransport::WebTransportOpt {
        listen: "0.0.0.0:4433".to_socket_addrs().unwrap().next().unwrap(),
        certs: Certs {
            key: "backend/certs/localhost.dev.key".into(),
            cert: "backend/certs/localhost.dev.pem".into(),
        },
    };

    let app = Router::new().route("/test", get(|| async { "Hello world" }));

    let app = app
        .layer(middleware::from_fn(mw_ctx_require))
        .nest(
            "/api/auth",
            routes_login::router(auth_service, session_service),
        )
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(
            session::Service::new(db.clone()),
            mw_ctx_resolver,
        ))
        .layer(middleware::from_fn(mw_req_stamp_resolver))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("listening on {}", "0.0.0.0:3000");
    tokio::select! {
        res = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle())).into_future() => {
            Ok(())
        },
        res = webtransport::start(opt).into_future() => {
            res
        }
    }?;

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
