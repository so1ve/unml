use std::path::Path;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{ProgressCallback, UnmlError};

/// Mod 平台（Modrinth、CurseForge 等）
#[async_trait]
pub trait ModPlatform: Send + Sync {
    type Error: UnmlError;

    /// 搜索 Mod
    async fn search_mods(
        &self,
        query: &str,
        filters: SearchFilters,
    ) -> Result<Vec<ModInfo>, Self::Error>;

    /// 获取 Mod 详细信息
    async fn get_mod(&self, mod_id: &str) -> Result<ModDetail, Self::Error>;

    /// 获取 Mod 的所有版本
    async fn get_mod_versions(&self, mod_id: &str) -> Result<Vec<ModVersion>, Self::Error>;

    /// 下载 Mod 文件
    async fn download_mod(
        &self,
        version: &ModVersion,
        dest: &Path,
        progress: Option<ProgressCallback>,
    ) -> Result<(), Self::Error>;
}

/// Mod 搜索过滤器
#[derive(Debug, Clone, Default)]
pub struct SearchFilters {
    pub game_version: Option<String>,
    pub mod_loader: Option<ModLoader>,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ModLoader {
    Forge,
    Fabric,
    NeoForge,
    Quilt,
}

/// Mod 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub downloads: u64,
}

/// Mod 详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModDetail {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub icon_url: Option<String>,
    pub downloads: u64,
    pub categories: Vec<String>,
}

/// Mod 版本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModVersion {
    pub id: String,
    pub version_number: String,
    pub game_versions: Vec<String>,
    pub loaders: Vec<ModLoader>,
    pub download_url: String,
    pub file_name: String,
}
