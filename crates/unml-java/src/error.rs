use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Java not found")]
    JavaNotFound,

    #[error("No suitable Java installation found (required version: {0})")]
    NoSuitableJava(u32),

    #[error("Failed to parse Java version")]
    VersionParseFailed,

    #[error("Invalid Java installation path")]
    InvalidJavaPath,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
