use tokio::sync::OnceCell;
use tower_cookies::Key;

pub mod error;
pub mod handler;
mod models;
pub mod mw_auth;
pub mod signed_cookies;

const SESSION_COOKIE_NAME: &str = "session-id";
pub static SESSION_COOKIE_KEY: OnceCell<Key> = OnceCell::const_new(); 
