use std::path::Path;

use async_trait::async_trait;
use unml_core::{Checksum, DownloadProvider, ProgressCallback, VersionInfo, VersionManifest};

use crate::{Error, Result};

pub struct MojangDownloadProvider;

impl MojangDownloadProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DownloadProvider for MojangDownloadProvider {
    type Error = Error;

    async fn fetch_version_manifest(&self) -> Result<VersionManifest> {
        let client = crate::http_client();
        let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| unml_core::HttpError(e.to_string()))?
            .error_for_status()
            .map_err(|e| unml_core::HttpError(e.to_string()))?;

        let manifest = response
            .json::<VersionManifest>()
            .await
            .map_err(|e| unml_core::HttpError(e.to_string()))?;

        Ok(manifest)
    }

    async fn fetch_version_info(&self, _version_id: &str) -> Result<VersionInfo> {
        // TODO: 实现
        todo!("fetch_version_info not implemented yet")
    }

    async fn download_file(
        &self,
        _url: &str,
        _dest: &Path,
        _checksum: Option<&Checksum>,
        _progress: Option<ProgressCallback>,
    ) -> Result<()> {
        // TODO: 实现
        todo!("download_file not implemented yet")
    }
}

impl Default for MojangDownloadProvider {
    fn default() -> Self {
        Self::new()
    }
}
