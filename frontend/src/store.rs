use yewdux::prelude::*;

#[derive(PartialEq, Clone)]
pub struct User {
    pub username: String,
}

#[derive(Clone, Default ,Store, PartialEq)]
pub struct Store {
    pub auth_user: Option<User>,
    pub is_loading: bool
}

