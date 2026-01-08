use std::path::Path;

use async_trait::async_trait;
use unml_core::{Checksum, DownloadProvider, VersionInfo, VersionManifest};

pub struct BMCLAPIDownloadProvider {
    base_provider: super::MojangDownloadProvider,
    mirror_root: &'static str,
}

impl BMCLAPIDownloadProvider {
    pub fn new() -> Self {
        Self {
            base_provider: super::MojangDownloadProvider::new(),
            mirror_root: "https://bmclapi2.bangbang93.com",
        }
    }
}

#[async_trait]
impl DownloadProvider for BMCLAPIDownloadProvider {
    async fn fetch_version_manifest(&self) -> unml_core::Result<VersionManifest> {
        self.base_provider.fetch_version_manifest().await
    }

    async fn fetch_version_info(&self, version_id: &str) -> unml_core::Result<VersionInfo> {
        self.base_provider.fetch_version_info(version_id).await
    }

    async fn download_file(
        &self,
        url: &str,
        dest: &Path,
        checksum: Option<&Checksum>,
        progress: Option<unml_core::ProgressCallback>,
    ) -> unml_core::Result<()> {
        let transformed_url = self.transform_url(url);
        self.base_provider
            .download_file(&transformed_url, dest, checksum, progress)
            .await
    }

    fn concurrency(&self) -> usize {
        num_cpus::get() * 2
    }

    fn transform_url(&self, url: &str) -> String {
        const REPLACEMENTS: &[(&str, &str)] = &[
            ("https://launchermeta.mojang.com", ""),
            ("https://piston-meta.mojang.com", ""),
            ("https://libraries.minecraft.net", "/libraries"),
        ];

        for (prefix, replacement) in REPLACEMENTS {
            if let Some(suffix) = url.strip_prefix(prefix) {
                return format!("{}{}{}", self.mirror_root, replacement, suffix);
            }
        }

        url.to_string()
    }
}

impl Default for BMCLAPIDownloadProvider {
    fn default() -> Self {
        Self::new()
    }
}
