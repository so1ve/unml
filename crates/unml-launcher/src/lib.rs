mod error;
mod launcher;
mod repository;

pub use error::{Error, Result};
pub use launcher::StandardLauncher;
pub use repository::FileSystemRepository;
// 重新导出 unml-java
pub use unml_java::{JavaDetector, JavaInstallation, JavaManager, JavaVersion};
