use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::sync::Arc;
use thiserror::Error;
use tracing::debug;

use crate::service;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Error, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    #[error(transparent)]
    Service(#[from] service::error::Error),

    // -- CtxExtError
    CtxExt(super::mw_auth::CtxExtError),

    // -- External Modules
    #[error(transparent)]
    SerdeJson(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        serde_json::Error,
    ),

    ReqStampNotInReqExt,

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    #[serde(skip)]
    AxumJsonRejection(#[from] JsonRejection),
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

// endregion: --- Error Boilerplate

// region:    --- Client Error

/// From the root error to the http status code and ClientError
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use service::error::Error::*;
        use Error::*;
        match self {
            Service(e) => match e {
                InvalidCredentials => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ClientError::SERVICE_ERROR,
                ),
            },

            // -- Auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    SERVICE_ERROR,
}
// endregion: --- Client Error
