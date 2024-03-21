pub mod auth;
pub mod email;
pub mod error;
pub mod room;

pub struct SearchResult<T>  {
    pub data: Vec<T>,
    pub next: Option<String>,
    pub prev: Option<String>
}

