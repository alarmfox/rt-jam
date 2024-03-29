use std::ops::Deref;

use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json as AJson, Router,
};
use serde_json::json;
use time::{Duration, OffsetDateTime};
use tower_cookies::{cookie::SameSite, Cookie};

use crate::service::user::{auth, session::{self, SessionData}, User};

use super::{
    error::Result, json::Json, mw_auth::CtxW, signed_cookies::Cookies, SESSION_COOKIE_NAME,
};

use common::types::{
    ChangePasswordRequest, LoginRequest, RegisterRequest, StartResetRequest, UserResponse,
};

#[derive(Clone)]
struct AppState {
    auth_service: auth::Service,
    session_service: session::Service,
}

pub fn router(auth_service: auth::Service, session_service: session::Service) -> Router {
    Router::new()
        .route("/sign-in", post(login))
        .route("/sign-up", post(register))
        .route("/sign-out", post(logout))
        .route("/me", get(me))
        .route("/change-password", post(change_password))
        .route("/start-reset", post(start_reset))
        .with_state(AppState {
            auth_service,
            session_service,
        })
}

async fn login(
    State(AppState {
        auth_service,
        session_service,
    }): State<AppState>,
    cookies: Cookies<'_>,
    Json(LoginRequest { username, password }): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    let user = auth_service.login(username, password).await?;
    let expiration = {
        let this = OffsetDateTime::now_utc();
        let duration = Duration::days(7);
        this.checked_add(duration)
            .expect("resulting value is out of range")
    };
    let token = session::Service::generate_token();
    session_service
        .create(
            &token,
            &session::SessionData::from(user.clone()),
            expiration,
        )
        .await?;

    let cookie = Cookie::build(Cookie::new(SESSION_COOKIE_NAME, token))
        .http_only(true)
        .secure(true)
        .expires(expiration)
        .path("/")
        .same_site(SameSite::Strict)
        .build();

    cookies.add(cookie);

    Ok(AJson(json!({
        "result": { "success": true }
    })))
}

async fn me(
    context: CtxW,
) -> Result<impl IntoResponse> {
    let session = context.0.get_session();
    let session = session.clone();

    Ok(AJson(UserResponse {
        id: session.id,
        email: session.email,
        first_name: session.first_name,
        last_name: session.last_name,
        username: session.username,
    }))
}

async fn logout(
    State(AppState {
        session_service, ..
    }): State<AppState>,
    context: CtxW,
) -> Result<impl IntoResponse> {
    session_service.delete(&context.0.get_token()).await?;
    Ok(AJson(json!({
        "result": { "success": true }
    })))
}

async fn register(
    State(AppState { auth_service, .. }): State<AppState>,
    Json(RegisterRequest {
        first_name,
        last_name,
        email,
        username,
    }): Json<RegisterRequest>,
) -> Result<impl IntoResponse> {
    let user = auth_service
        .register(first_name, last_name, email, username)
        .await?;

    Ok(AJson(UserResponse::from(user)))
}

async fn change_password(
    State(AppState { auth_service, .. }): State<AppState>,
    Json(ChangePasswordRequest {
        token, password, ..
    }): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse> {
    auth_service.reset_password(password, token).await?;

    Ok(AJson(json!({
        "result": { "success": true }
    })))
}
async fn start_reset(
    State(AppState { auth_service, .. }): State<AppState>,
    Json(StartResetRequest { email }): Json<StartResetRequest>,
) -> Result<impl IntoResponse> {
    auth_service.start_password_reset(email).await?;

    Ok(AJson(json!({
        "result": { "success": true }
    })))
}
impl From<User> for UserResponse {
    fn from(
        User {
            id,
            email,
            first_name,
            last_name,
            username,
            created_at,
            updated_at,
            ..
        }: User,
    ) -> Self {
        Self {
            id,
            email,
            first_name,
            last_name,
            username,
        }
    }
}
