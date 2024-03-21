use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use tower_cookies::{Cookie, SignedCookies};
use tracing::debug;

use crate::web::SESSION_COOKIE_KEY;

use super::error::Error;


pub struct Cookies<'a>(pub SignedCookies<'a>);

impl Cookies<'_> {
    pub fn remove(&self, cookie: Cookie<'static>) {
        self.0.remove(cookie)
    }
    pub fn add(&self, cookie: Cookie<'static>) {
        self.0.add(cookie)
    }
    pub fn get(&self, v: &str) -> Option<Cookie<'_>> {
        self.0.get(v)
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Cookies<'_> {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Error> {
        debug!("{:<12} - Signed Cookies", "EXTRACTOR");
        let key = SESSION_COOKIE_KEY
            .get()
            .expect("SESSION KEY IS NOT INITIALIZED");
        let cookies = tower_cookies::Cookies::from_request_parts(parts, state)
            .await
            .unwrap()
            .signed(key);
        Ok(Cookies(cookies))
    }
}
