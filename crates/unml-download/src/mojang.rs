use std::path::Path;

use async_trait::async_trait;
use unml_core::{Checksum, DownloadProvider, VersionInfo, VersionManifest};

pub struct MojangDownloadProvider;

impl MojangDownloadProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DownloadProvider for MojangDownloadProvider {
    async fn fetch_version_manifest(&self) -> unml_core::Result<VersionManifest> {
        let client = crate::http_client();
        let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| unml_core::Error::Http(e.to_string()))?
            .error_for_status()
            .map_err(|e| unml_core::Error::Http(e.to_string()))?;

        let manifest = response
            .json::<VersionManifest>()
            .await
            .map_err(|e| unml_core::Error::Http(e.to_string()))?;

        Ok(manifest)
    }

    async fn fetch_version_info(&self, _version_id: &str) -> unml_core::Result<VersionInfo> {
        // TODO: 实现
        todo!("fetch_version_info not implemented yet")
    }

    async fn download_file(
        &self,
        _url: &str,
        _dest: &Path,
        _checksum: Option<&Checksum>,
        _progress: Option<unml_core::ProgressCallback>,
    ) -> unml_core::Result<()> {
        // TODO: 实现
        todo!("download_file not implemented yet")
    }
}

impl Default for MojangDownloadProvider {
    fn default() -> Self {
        Self::new()
    }
}
