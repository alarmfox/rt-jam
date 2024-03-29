use crate::service::user::session::SessionData;

#[derive(Debug, Clone)]
pub struct Context {
    data: SessionData,
    token: String,
}

impl Context {
    pub fn new(data: SessionData, token: String) -> Self {
        Self { data, token }
    }

    pub fn get_session(&self) -> SessionData {
        self.data.clone()
    }
    pub fn get_token(&self) -> String {
        self.token.clone()
    }
}
