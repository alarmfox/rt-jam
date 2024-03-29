use common::types::UserResponse;
use yewdux::prelude::*;

#[derive(PartialEq, Clone)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Clone, Default, Store, PartialEq)]
pub struct Store {
    pub auth_user: Option<User>,
    pub is_loading: bool,
}

impl From<UserResponse> for User {
    fn from(
        UserResponse {
            id,
            email,
            first_name,
            last_name,
            username,
            ..
        }: UserResponse,
    ) -> Self {
        Self {
            username,
            first_name,
            last_name,
            email,
        }
    }
}
