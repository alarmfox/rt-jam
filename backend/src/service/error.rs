use axum::http::StatusCode;
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::api::auth;

#[serde_as]
#[derive(Debug, Serialize, Error, From)]
pub enum Error {
    #[error(transparent)]
    AuthError(auth::error::Error),

    #[error(transparent)]
    DatabaseError(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

    #[error(transparent)]
    SerializationError(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        todo!()
	}
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	INVALID_PARAMS,
	SERVICE_ERROR,
}
