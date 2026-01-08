use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Version not found: {0}")]
    VersionNotFound(String),

    #[error("Checksum mismatch: expected {expected}, got {calculated}")]
    ChecksumMismatch {
        expected: String,
        calculated: String,
    },

    #[error("Cache not found: {0}")]
    CacheNotFound(String),

    #[error("Java not found")]
    JavaNotFound,

    #[error("Java version parse failed")]
    JavaVersionParseFailed,

    #[error("Invalid Java path")]
    InvalidJavaPath,

    #[error("No suitable Java installation found (required version: {0})")]
    NoSuitableJava(u32),

    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;
