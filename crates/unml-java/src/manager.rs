use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{Error, JavaDetector, JavaInstallation, JavaVersion, Result};

pub struct JavaManager {
    detector: JavaDetector,
    cache: Arc<RwLock<Option<Vec<JavaInstallation>>>>,
}

impl JavaManager {
    pub fn new() -> Self {
        Self {
            detector: JavaDetector::new(),
            cache: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_installations(&self) -> Result<Vec<JavaInstallation>> {
        let cache = self.cache.read().await;
        if let Some(ref installations) = *cache {
            return Ok(installations.clone());
        }
        drop(cache);

        self.refresh().await
    }

    pub async fn refresh(&self) -> Result<Vec<JavaInstallation>> {
        let installations = self.detector.detect().await?;

        let mut cache = self.cache.write().await;
        *cache = Some(installations.clone());

        Ok(installations)
    }

    pub async fn find_best(&self, required: JavaVersion) -> Result<JavaInstallation> {
        let installations = self.get_installations().await?;

        installations
            .into_iter()
            .find(|i| i.satisfies(required))
            .ok_or(Error::NoSuitableJava(required.major))
    }

    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        *cache = None;
    }
}

impl Default for JavaManager {
    fn default() -> Self {
        Self::new()
    }
}
