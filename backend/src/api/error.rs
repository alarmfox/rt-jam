use std::sync::Arc;

use axum::{
    extract::rejection::FormRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_with::serde_as;
use strum_macros::AsRefStr;
use thiserror::Error;
use validator::ValidationErrorsKind;

use crate::service::error::Error as ServiceError;

use super::auth;

#[serde_as]
#[derive(Debug, Serialize, Error, AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    #[serde(skip)]
    AxumFormRejection(#[from] FormRejection),

    #[error(transparent)]
    Auth(#[from] auth::error::Error),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let mut res = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        res.extensions_mut().insert(Arc::new(self));
        res
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        todo!()
    }
}

fn make_error_message(e: validator::ValidationErrors) -> String {
    let e = e.errors();
    let e = e.values().next().unwrap();
    match e {
        ValidationErrorsKind::Field(e) => e.first().unwrap().to_string(),
        _ => todo!(),
    }
}
