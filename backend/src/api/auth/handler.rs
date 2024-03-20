use std::ops::Add;

use axum::{
    extract::{FromRef, State},
    http::StatusCode,
    response::{IntoResponse},
    Json, Router,
};
use serde_json::json;
use time::{Duration, OffsetDateTime};
use tower_cookies::{cookie::SameSite, Cookie};

use crate::{
    api::{context::Context, form::Form},
    service::auth::{auth, session},
};

use super::{
    error::Result,
    models::{LoginPayload, RegisterPayload, ResetPayload, StartResetPayload},
    signed_cookies::Cookies,
    SESSION_COOKIE_NAME,
};

#[derive(Clone, FromRef)]
struct AppState {
    session_service: session::Service,
    auth_service: auth::Service,
}

pub fn routes(session_service: session::Service, auth_service: auth::Service) -> Router {
    let app_state = AppState {
        session_service,
        auth_service,
    };
    Router::new().with_state(app_state)
}
struct Login {}
impl Login {
    async fn post(
        State(auth_service): State<auth::Service>,
        State(session_service): State<session::Service>,
        cookies: Cookies<'_>,
        Form(form): Form<LoginPayload>,
    ) -> Result<impl IntoResponse> {
        let user = auth_service.login(form.username, form.password).await?;

        let expiration = OffsetDateTime::now_utc().add(Duration::days(7));
        let token = session::Service::generate_token();
        session_service
            .create(&token, &session::SessionData::from(user), expiration)
            .await?;

        let cookie = Cookie::build(Cookie::new(SESSION_COOKIE_NAME, token))
            .http_only(true)
            .secure(true)
            .expires(expiration)
            .path("/")
            .same_site(SameSite::Strict)
            .build();

        cookies.add(cookie);

        Ok(Json(json!({
            "result": { "success": true }
        })))
    }
}

struct Register {}
impl Register {
    async fn post(
        State(auth_service): State<auth::Service>,
        Form(RegisterPayload {
            first_name,
            last_name,
            email,
            username,
        }): Form<RegisterPayload>,
    ) -> Result<impl IntoResponse> {
        auth_service
            .register(first_name, last_name, email, username)
            .await?;

        Ok(())
    }
}

async fn logout(
    context: Context,
    State(session_service): State<session::Service>,
    cookies: Cookies<'_>,
) -> Result<impl IntoResponse> {
    session_service.delete(context.get_token().as_str()).await?;
    let cookie = Cookie::build(Cookie::new(SESSION_COOKIE_NAME, ""))
        .http_only(true)
        .secure(true)
        .path("/")
        .same_site(SameSite::Strict)
        .build();
    cookies.remove(cookie);

    Ok(StatusCode::NO_CONTENT)
}

struct Reset {}

impl Reset {
    async fn post(
        State(auth_service): State<auth::Service>,
        Form(form): Form<StartResetPayload>,
    ) -> Result<impl IntoResponse> {
        auth_service.start_password_reset(form.email).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}

struct ChangePassword {}

impl ChangePassword {
    async fn post(
        State(auth_service): State<auth::Service>,
        Form(form): Form<ResetPayload>,
    ) -> Result<impl IntoResponse> {
        auth_service
            .reset_password(form.password, form.token)
            .await?;

        Ok(StatusCode::NO_CONTENT)
    }
}
