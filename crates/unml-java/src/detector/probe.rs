use std::path::Path;

use tokio::process::Command;

use crate::{Error, JavaInstallation, Result};

pub struct JavaProbe;

impl JavaProbe {
    pub async fn probe(executable: &Path) -> Result<JavaInstallation> {
        if !executable.exists() {
            return Err(Error::JavaNotFound);
        }

        let output = Command::new(executable).arg("-version").output().await?;

        let stderr = String::from_utf8_lossy(&output.stderr);

        let version = Self::parse_version(&stderr)?;
        let major_version = Self::extract_major_version(&version)?;
        let vendor = Self::parse_vendor(&stderr);
        let arch = Self::parse_arch(&stderr);

        let home = executable
            .parent()
            .and_then(|p| p.parent())
            .ok_or(Error::InvalidJavaPath)?
            .to_path_buf();

        Ok(JavaInstallation {
            executable: executable.to_path_buf(),
            version,
            major_version,
            vendor,
            home,
            arch,
        })
    }

    fn parse_version(output: &str) -> Result<String> {
        for line in output.lines() {
            if line.contains("version")
                && let Some(start) = line.find('"')
                && let Some(end) = line[start + 1..].find('"')
            {
                return Ok(line[start + 1..start + 1 + end].to_string());
            }
        }

        Err(Error::VersionParseFailed)
    }

    fn extract_major_version(version: &str) -> Result<u32> {
        // "1.8.0_292" -> 8
        // "17.0.1" -> 17
        let stripped = version.strip_prefix("1.").unwrap_or(version);
        let parts: Vec<&str> = stripped.split('.').collect();
        if !parts.is_empty() {
            return parts[0].parse().map_err(|_| Error::VersionParseFailed);
        }

        Err(Error::VersionParseFailed)
    }

    fn parse_vendor(output: &str) -> Option<String> {
        for line in output.lines() {
            let lower = line.to_lowercase();

            if lower.contains("openjdk") {
                return Some("OpenJDK".to_string());
            } else if lower.contains("oracle") {
                return Some("Oracle".to_string());
            } else if lower.contains("zulu") {
                return Some("Azul Zulu".to_string());
            } else if lower.contains("adoptium") || lower.contains("temurin") {
                return Some("Eclipse Adoptium".to_string());
            }
        }

        None
    }

    fn parse_arch(output: &str) -> Option<String> {
        for line in output.lines() {
            if line.contains("64-Bit") || line.contains("x86_64") || line.contains("amd64") {
                return Some("x64".to_string());
            } else if line.contains("aarch64") || line.contains("arm64") {
                return Some("aarch64".to_string());
            }
        }

        None
    }
}
