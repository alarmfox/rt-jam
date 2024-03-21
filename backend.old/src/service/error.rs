use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::api::auth;

#[serde_as]
#[derive(Debug, Serialize, Error, From)]
pub enum Error {
    #[error(transparent)]
    AuthError(#[serde_as(as = "DisplayFromStr")] auth::error::Error),

    #[error(transparent)]
    DatabaseError(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

    #[error(transparent)]
    SerializationError(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}
