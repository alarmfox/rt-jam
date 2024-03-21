use crate::service::user::session;
use crate::web::SESSION_COOKIE_NAME;
use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use super::context::Context;
use super::error::{Error, Result};

pub async fn mw_ctx_require(ctx: Result<CtxW>, req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

// IMPORTANT: This resolver must never fail, but rather capture the potential Auth error and put in in the
//            request extension as CtxExtResult.
//            This way it won't prevent downstream middleware to be executed, and will still capture the error
//            for the appropriate middleware (.e.g., mw_ctx_require which forces successful auth) or handler
//            to get the appropriate information.
pub async fn mw_ctx_resolver(
    State(service): State<session::Service>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = ctx_resolve(service, &cookies).await;

    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(SESSION_COOKIE_NAME))
    }

    // Store the ctx_ext_result in the request extension
    // (for Ctx extractor).
    req.extensions_mut().insert(ctx_ext_result);

    next.run(req).await
}

async fn ctx_resolve(session_service: session::Service, cookies: &Cookies) -> CtxExtResult {
    match cookies
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)
    {
        Ok(s) => match session_service.get(s.clone()).await {
            Ok(Some(session)) => Ok(CtxW(Context::new(session, s))),
            Ok(None) => Err(CtxExtError::UserNotFound),
            Err(e) => Err(CtxExtError::SessionError(e.to_string())),
        },
        Err(e) => Err(e),
    }
}
// region:    --- Ctx Extractor
#[derive(Debug, Clone)]
pub struct CtxW(pub Context);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<CtxW, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,

    UserNotFound,
    FailValidate,
    CannotSetTokenCookie,

    CtxNotInRequestExt,
    CtxCreateFail(String),
    SessionError(String),
}
// endregion: --- Ctx Extractor Result/Error
