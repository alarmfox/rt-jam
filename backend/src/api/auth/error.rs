use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

use crate::service::auth;

use super::mw_auth;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error(transparent)]
    CtxExt(#[from]mw_auth::CtxExtError),

    #[error(transparent)]
    ServiceError(#[from] auth::error::Error),
}


impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::CtxExt(_) => (StatusCode::UNAUTHORIZED, "Not authorized").into_response(),
            Error::ServiceError(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}
