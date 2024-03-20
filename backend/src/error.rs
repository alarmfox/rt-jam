#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum Error {
    InvalidConfig { detail: String },
    KeyError { detail: String },
}

