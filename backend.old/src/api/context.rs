use std::sync::Arc;

use crate::service::auth::session::SessionData;

#[derive(Debug, Clone)]
pub struct Context {
    data: Arc<SessionData>,
    token: String,
}

impl Context {
    pub fn new(data: SessionData, token: String) -> Self {
        Self {
            data: Arc::new(data),
            token,
        }
    }

    pub fn get_session(&self) -> Arc<SessionData> {
        self.data.clone()
    }
    pub fn get_token(&self) -> String {
        self.token.clone()
    }
}
