mod detector;
mod error;
mod installation;
mod manager;
mod version;

pub use detector::JavaDetector;
pub use error::{Error, Result};
pub use installation::JavaInstallation;
pub use manager::JavaManager;
pub use version::JavaVersion;
