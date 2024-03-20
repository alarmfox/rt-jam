use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use serde::Serialize;
use strum_macros::Display;
use thiserror::Error;
use tower_cookies::Cookie;

use crate::{
    api::{auth::SESSION_COOKIE_NAME, context::Context},
    service::auth::session,
};

use super::{
    error::{Error, Result},
    signed_cookies::Cookies,
};

pub async fn mw_require_auth(
    ctx: Result<Context>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    State(session_service): State<session::Service>,
    cookies: Cookies<'_>,
    mut req: Request,
    next: Next,
) -> Response {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let result_ctx = ctx_resolve(session_service, &cookies).await;

    // Remove the cookie if something went wrong other than NoAuth.
    if result_ctx.is_err() && !matches!(result_ctx, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(SESSION_COOKIE_NAME))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    next.run(req).await
}

async fn ctx_resolve(session_service: session::Service, cookies: &Cookies<'_>) -> CtxExtResult {
    match cookies
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)
    {
        Ok(s) => match session_service.get(s.clone()).await {
            Ok(Some(session)) => Ok(Context::new(session, s)),
            Ok(None) => Err(CtxExtError::UserNotFound),
            Err(e) => Err(CtxExtError::SessionError(e.to_string())),
        },
        Err(e) => Err(e),
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(CtxExtError::CtxNotInRequestExt)?
            .clone()
            .map_err(Error::CtxExt)
    }
}

type CtxExtResult = core::result::Result<Context, CtxExtError>;

#[derive(Clone, Error, Display, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,

    UserNotFound,

    CtxNotInRequestExt,
    CtxCreateFail(String),
    SessionError(String)
}
