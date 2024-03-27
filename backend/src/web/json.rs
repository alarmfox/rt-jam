use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json as AxumJson, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use super::error::Error;

pub struct Json<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for Json<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    AxumJson<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let AxumJson(v) = AxumJson::<T>::from_request(req, state).await?;
        v.validate()?;
        Ok(Json(v))
    }
}
