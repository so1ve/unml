use std::borrow::Cow;
use std::path::Path;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::UnmlError;

pub type ProgressCallback = Box<dyn Fn(u64, u64) + Send + Sync>;

/// 下载提供者的核心 trait
#[async_trait]
pub trait DownloadProvider: Send + Sync {
    type Error: UnmlError;

    /// 获取版本清单
    async fn fetch_version_manifest(&self) -> Result<VersionManifest, Self::Error>;

    /// 获取指定版本的详细信息
    async fn fetch_version_info(&self, version_id: &str) -> Result<VersionInfo, Self::Error>;

    /// 下载文件，支持进度回调
    async fn download_file(
        &self,
        url: &str,
        dest: &Path,
        checksum: Option<&Checksum>,
        progress: Option<ProgressCallback>,
    ) -> Result<(), Self::Error>;

    /// 获取并发数配置
    fn concurrency(&self) -> usize {
        6
    }

    fn transform_url<'a>(&self, url: &'a str) -> Cow<'a, str> {
        Cow::Borrowed(url)
    }
}

/// 校验和
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Checksum {
    Sha1(String),
    Sha256(String),
}

/// 版本清单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersions,
    pub versions: Vec<Version>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

/// 版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

/// 详细版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    pub libraries: Vec<Library>,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    #[serde(rename = "minecraftArguments", default)]
    pub minecraft_arguments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    pub downloads: Option<LibraryDownloads>,
}

impl Library {
    pub fn is_allowed(&self) -> bool {
        // 简化版本，实际需要检查 rules
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<Artifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub url: String,
    pub sha1: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndex {
    pub id: String,
    pub url: String,
    pub sha1: String,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
}
