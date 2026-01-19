use crate::detector::scanner::DirectoryScanner;
use crate::{JavaInstallation, Result};

pub struct LinuxDetector;

impl LinuxDetector {
    pub async fn detect_from_system() -> Result<Vec<JavaInstallation>> {
        let mut installations = Vec::new();

        let bases = vec!["/usr/lib/jvm", "/usr/java", "/opt/java", "/usr/lib64/jvm"];

        for base in bases {
            if let Ok(found) = DirectoryScanner::scan(base).await {
                installations.extend(found);
            }
        }

        Ok(installations)
    }
}
