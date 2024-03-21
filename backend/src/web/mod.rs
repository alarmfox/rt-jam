use tokio::sync::OnceCell;
use tower_cookies::Key;

pub mod context;
pub mod error;
pub mod form;
pub mod mw_auth;
pub mod mw_res_map;
pub mod mw_req_stamp;
pub mod routes_login;
pub mod signed_cookies;

pub const SESSION_COOKIE_NAME: &str = "session-id";
pub static SESSION_COOKIE_KEY: OnceCell<Key> = OnceCell::const_new(); 
