use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Mod not found: {0}")]
    ModNotFound(String),

    #[error("Version not found: {0}")]
    VersionNotFound(String),

    #[error(transparent)]
    Io(#[from] unml_core::IoError),

    #[error(transparent)]
    Http(#[from] unml_core::HttpError),

    #[error(transparent)]
    Json(#[from] unml_core::JsonError),
}

pub type Result<T> = std::result::Result<T, Error>;
