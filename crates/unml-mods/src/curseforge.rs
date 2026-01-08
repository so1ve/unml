use std::path::Path;

use async_trait::async_trait;
use unml_core::{ModDetail, ModInfo, ModPlatform, ModVersion, SearchFilters};

pub struct CurseForgePlatform;

impl CurseForgePlatform {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ModPlatform for CurseForgePlatform {
    async fn search_mods(
        &self,
        _query: &str,
        _filters: SearchFilters,
    ) -> unml_core::Result<Vec<ModInfo>> {
        Ok(Vec::new())
    }

    async fn get_mod(&self, _mod_id: &str) -> unml_core::Result<ModDetail> {
        todo!("get_mod not implemented yet")
    }

    async fn get_mod_versions(&self, _mod_id: &str) -> unml_core::Result<Vec<ModVersion>> {
        Ok(Vec::new())
    }

    async fn download_mod(
        &self,
        _version: &ModVersion,
        _dest: &Path,
        _progress: Option<unml_core::ProgressCallback>,
    ) -> unml_core::Result<()> {
        Ok(())
    }
}

impl Default for CurseForgePlatform {
    fn default() -> Self {
        Self::new()
    }
}
