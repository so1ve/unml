use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Version not found: {0}")]
    VersionNotFound(String),

    #[error("Launch failed: {0}")]
    LaunchFailed(String),

    #[error("Game directory not found")]
    GameDirNotFound,

    #[error(transparent)]
    Io(#[from] unml_core::IoError),

    #[error(transparent)]
    Json(#[from] unml_core::JsonError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(unml_core::IoError(e))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
