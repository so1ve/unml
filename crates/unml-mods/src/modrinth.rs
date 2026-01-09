use std::path::Path;

use async_trait::async_trait;
use unml_core::{ModDetail, ModInfo, ModPlatform, ModVersion, ProgressCallback, SearchFilters};

use crate::{Error, Result};

pub struct ModrinthPlatform;

impl ModrinthPlatform {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ModPlatform for ModrinthPlatform {
    type Error = Error;

    async fn search_mods(&self, _query: &str, _filters: SearchFilters) -> Result<Vec<ModInfo>> {
        // TODO: 实现
        Ok(Vec::new())
    }

    async fn get_mod(&self, _mod_id: &str) -> Result<ModDetail> {
        // TODO: 实现
        todo!("get_mod not implemented yet")
    }

    async fn get_mod_versions(&self, _mod_id: &str) -> Result<Vec<ModVersion>> {
        // TODO: 实现
        Ok(Vec::new())
    }

    async fn download_mod(
        &self,
        _version: &ModVersion,
        _dest: &Path,
        _progress: Option<ProgressCallback>,
    ) -> Result<()> {
        // TODO: 实现
        Ok(())
    }
}

impl Default for ModrinthPlatform {
    fn default() -> Self {
        Self::new()
    }
}
