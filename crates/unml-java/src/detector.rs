use std::path::{Path, PathBuf};

use tokio::process::Command;

use crate::{Error, JavaInstallation, Result};

pub struct JavaDetector;

impl JavaDetector {
    pub fn new() -> Self {
        Self
    }

    /// 检测系统中所有 Java 安装
    pub async fn detect_all(&self) -> Result<Vec<JavaInstallation>> {
        let mut installations = Vec::new();
        let mut seen_paths = std::collections::HashSet::new();

        // 1. 检查 JAVA_HOME
        if let Some(installation) = self.detect_from_env().await? {
            seen_paths.insert(installation.executable.clone());
            installations.push(installation);
        }

        // 2. 检查 PATH
        for installation in self.detect_from_path().await? {
            if seen_paths.insert(installation.executable.clone()) {
                installations.push(installation);
            }
        }

        // 3. 检查系统常见位置
        for installation in self.detect_from_system().await? {
            if seen_paths.insert(installation.executable.clone()) {
                installations.push(installation);
            }
        }

        // 按主版本号排序（降序）
        installations.sort_by(|a, b| b.major_version.cmp(&a.major_version));

        Ok(installations)
    }

    /// 查找满足版本要求的 Java
    pub async fn find_suitable(&self, required: crate::JavaVersion) -> Result<JavaInstallation> {
        let installations = self.detect_all().await?;

        installations
            .into_iter()
            .find(|i| i.satisfies(required))
            .ok_or(Error::NoSuitableJava(required.major))
    }

    /// 从 JAVA_HOME 环境变量检测
    async fn detect_from_env(&self) -> Result<Option<JavaInstallation>> {
        let java_home = match std::env::var("JAVA_HOME") {
            Ok(path) => PathBuf::from(path),
            Err(_) => return Ok(None),
        };

        let executable = java_home
            .join("bin")
            .join(JavaInstallation::executable_name());

        match self.probe(&executable).await {
            Ok(mut installation) => {
                installation.home = java_home;
                Ok(Some(installation))
            }
            Err(_) => Ok(None),
        }
    }

    /// 从 PATH 环境变量检测
    async fn detect_from_path(&self) -> Result<Vec<JavaInstallation>> {
        let path = match std::env::var("PATH") {
            Ok(p) => p,
            Err(_) => return Ok(Vec::new()),
        };

        let separator = if cfg!(windows) { ";" } else { ":" };
        let mut installations = Vec::new();

        for dir in path.split(separator) {
            let executable = PathBuf::from(dir).join(JavaInstallation::executable_name());
            if let Ok(installation) = self.probe(&executable).await {
                installations.push(installation);
            }
        }

        Ok(installations)
    }

    /// 从系统常见位置检测
    async fn detect_from_system(&self) -> Result<Vec<JavaInstallation>> {
        let mut installations = Vec::new();

        #[cfg(windows)]
        {
            let bases = vec![
                "C:\\Program Files\\Java",
                "C:\\Program Files (x86)\\Java",
                "C:\\Program Files\\Eclipse Adoptium",
                "C:\\Program Files\\Zulu",
            ];

            for base in bases {
                if let Ok(found) = self.scan_directory(base).await {
                    installations.extend(found);
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let bases = vec![
                "/Library/Java/JavaVirtualMachines",
                "/System/Library/Java/JavaVirtualMachines",
            ];

            for base in bases {
                if let Ok(found) = self.scan_directory_macos(base).await {
                    installations.extend(found);
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            let bases = vec!["/usr/lib/jvm", "/usr/java", "/opt/java", "/usr/lib64/jvm"];

            for base in bases {
                if let Ok(found) = self.scan_directory(base).await {
                    installations.extend(found);
                }
            }
        }

        Ok(installations)
    }

    /// 扫描目录查找 Java 安装
    async fn scan_directory(&self, base: impl AsRef<Path>) -> Result<Vec<JavaInstallation>> {
        let mut installations = Vec::new();
        let base = base.as_ref();

        if !tokio::fs::try_exists(base).await.unwrap_or(false) {
            return Ok(installations);
        }

        let mut entries = tokio::fs::read_dir(base).await?;

        while let Some(entry) = entries.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let java_home = entry.path();
            let executable = java_home
                .join("bin")
                .join(JavaInstallation::executable_name());

            if let Ok(mut installation) = self.probe(&executable).await {
                installation.home = java_home;
                installations.push(installation);
            }
        }

        Ok(installations)
    }

    /// macOS 特殊处理（Contents/Home 子目录）
    #[cfg(target_os = "macos")]
    async fn scan_directory_macos(&self, base: impl AsRef<Path>) -> Result<Vec<JavaInstallation>> {
        let mut installations = Vec::new();
        let base = base.as_ref();

        if !tokio::fs::try_exists(base).await.unwrap_or(false) {
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

            if let Ok(mut installation) = self.probe(&executable).await {
                installation.home = java_home;
                installations.push(installation);
            }
        }

        Ok(installations)
    }

    /// 探测特定 Java 可执行文件
    async fn probe(&self, executable: &Path) -> Result<JavaInstallation> {
        if !tokio::fs::try_exists(executable).await? {
            return Err(Error::JavaNotFound);
        }

        // 执行 java -version
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
        if let Some(stripped) = version.strip_prefix("1.") {
            let parts: Vec<&str> = stripped.split('.').collect();
            if !parts.is_empty() {
                return parts[0].parse().map_err(|_| Error::VersionParseFailed);
            }
        } else {
            let parts: Vec<&str> = version.split('.').collect();
            if !parts.is_empty() {
                return parts[0].parse().map_err(|_| Error::VersionParseFailed);
            }
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

impl Default for JavaDetector {
    fn default() -> Self {
        Self::new()
    }
}
