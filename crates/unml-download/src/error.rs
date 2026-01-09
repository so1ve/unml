use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Version not found: {0}")]
    VersionNotFound(String),

    #[error(transparent)]
    Io(#[from] unml_core::IoError),

    #[error(transparent)]
    Http(#[from] unml_core::HttpError),

    #[error(transparent)]
    Json(#[from] unml_core::JsonError),

    #[error(transparent)]
    Checksum(#[from] unml_core::ChecksumError),
}

pub type Result<T> = std::result::Result<T, Error>;
