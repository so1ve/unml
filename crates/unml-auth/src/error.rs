use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error(transparent)]
    Http(#[from] unml_core::HttpError),

    #[error(transparent)]
    Json(#[from] unml_core::JsonError),
}

pub type Result<T> = std::result::Result<T, Error>;
