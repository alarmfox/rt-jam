use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Error)]
pub enum Error {
    CryptoError,

    EmailError,

    #[error(transparent)]
    DatabaseError(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        sqlx::Error,
    ),

    InvalidCredentials,
    NoAuth,

    #[error(transparent)]
    SerializationError(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
    UserAlreadyExists,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
