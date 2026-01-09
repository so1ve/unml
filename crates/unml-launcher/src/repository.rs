use std::path::PathBuf;

use async_trait::async_trait;
use unml_core::GameRepository;

use crate::{Error, Result};

pub struct FileSystemRepository {
    root: PathBuf,
}

impl FileSystemRepository {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

#[async_trait]
impl GameRepository for FileSystemRepository {
    type Error = Error;

    async fn list_installed_versions(&self) -> Result<Vec<String>> {
        // TODO: 实现
        Ok(Vec::new())
    }

    async fn verify_version(&self, _version_id: &str) -> Result<bool> {
        // TODO: 实现
        Ok(true)
    }

    fn get_version_path(&self, version_id: &str) -> PathBuf {
        self.root.join("versions").join(version_id)
    }

    fn get_version_json(&self, version_id: &str) -> PathBuf {
        self.get_version_path(version_id)
            .join(format!("{}.json", version_id))
    }

    fn get_version_jar(&self, version_id: &str) -> PathBuf {
        self.get_version_path(version_id)
            .join(format!("{}.jar", version_id))
    }
}
