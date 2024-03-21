use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use strum_macros::Display;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Display, Error, Serialize)]
pub enum Error {
    CryptoError,

    #[error(transparent)]
    DatabaseError(#[from] #[serde_as(as = "DisplayFromStr")]sqlx::Error),

    InvalidCredentials,
    NoAuth,

    #[error(transparent)]
    SerializationError(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
    UserAlreadyExists,
}
