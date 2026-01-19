use std::path::Path;

use crate::detector::probe::JavaProbe;
use crate::{JavaInstallation, Result};

pub struct MacOSDetector;

impl MacOSDetector {
    pub async fn detect_from_system() -> Result<Vec<JavaInstallation>> {
        let mut installations = Vec::new();

        let bases = vec![
            "/Library/Java/JavaVirtualMachines",
            "/System/Library/Java/JavaVirtualMachines",
        ];

        for base in bases {
            if let Ok(found) = Self::scan_macos(base).await {
                installations.extend(found);
            }
        }

        Ok(installations)
    }

    async fn scan_macos(base: impl AsRef<Path>) -> Result<Vec<JavaInstallation>> {
        let mut installations = Vec::new();
        let base = base.as_ref();

        if !base.exists() {
            return Ok(installations);
        }

        let mut entries = tokio::fs::read_dir(base).await?;

        while let Some(entry) = entries.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let java_home = entry.path().join("Contents/Home");
            let executable = java_home
                .join("bin")
                .join(JavaInstallation::executable_name());

            if let Ok(mut installation) = JavaProbe::probe(&executable).await {
                installation.home = java_home;
                installations.push(installation);
            }
        }

        Ok(installations)
    }
}
