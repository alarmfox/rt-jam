use async_trait::async_trait;
use axum::extract::{rejection::FormRejection, Form as AxumForm, FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

use super::error::Error;

pub struct Form<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for Form<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    AxumForm<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let AxumForm(v) = AxumForm::<T>::from_request(req, state).await?;
        v.validate()?;
        Ok(Form(v))
    }
}
