mod platform;
mod probe;
mod scanner;

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[cfg(target_os = "linux")]
use self::platform::LinuxDetector;
#[cfg(target_os = "macos")]
use self::platform::MacOSDetector;
#[cfg(windows)]
use self::platform::WindowsDetector;
use self::probe::JavaProbe;
use crate::{Error, JavaInstallation, JavaVersion, Result};

pub struct JavaDetector;

impl JavaDetector {
    pub fn new() -> Self {
        Self
    }

    pub async fn detect(&self) -> Result<Vec<JavaInstallation>> {
        let mut installations = Vec::new();
        let mut seen_paths = HashSet::new();

        // 1. JAVA_HOME
        if let Some(installation) = self.detect_from_env().await? {
            let canonical = self.get_canonical_path(&installation.executable);
            if seen_paths.insert(canonical) {
                installations.push(installation);
            }
        }

        // 2. PATH
        for installation in self.detect_from_path().await? {
            let canonical = self.get_canonical_path(&installation.executable);
            if seen_paths.insert(canonical) {
                installations.push(installation);
            }
        }

        // 3. System locations
        for installation in self.detect_from_system().await? {
            let canonical = self.get_canonical_path(&installation.executable);
            if seen_paths.insert(canonical) {
                installations.push(installation);
            }
        }

        installations.retain(|inst| self.is_valid_installation(inst));

        // descending
        installations.sort_by(|a, b| b.major_version.cmp(&a.major_version));

        Ok(installations)
    }

    pub async fn find_suitable(&self, required: JavaVersion) -> Result<JavaInstallation> {
        let installations = self.detect().await?;

        installations
            .into_iter()
            .find(|i| i.satisfies(required))
            .ok_or(Error::NoSuitableJava(required.major))
    }

    async fn detect_from_env(&self) -> Result<Option<JavaInstallation>> {
        let java_home = match env::var("JAVA_HOME") {
            Ok(path) => PathBuf::from(path),
            Err(_) => return Ok(None),
        };

        let executable = java_home
            .join("bin")
            .join(JavaInstallation::executable_name());

        match JavaProbe::probe(&executable).await {
            Ok(mut installation) => {
                installation.home = java_home;

                Ok(Some(installation))
            }
            Err(_) => Ok(None),
        }
    }

    async fn detect_from_path(&self) -> Result<Vec<JavaInstallation>> {
        let path = match std::env::var("PATH") {
            Ok(p) => p,
            Err(_) => return Ok(Vec::new()),
        };

        let separator = if cfg!(windows) { ";" } else { ":" };

        let candidates: Vec<PathBuf> = path
            .split(separator)
            .map(|dir| PathBuf::from(dir).join(JavaInstallation::executable_name()))
            .filter(|exe| exe.exists())
            .collect();

        let mut handles = Vec::with_capacity(candidates.len());
        for executable in candidates {
            handles.push(tokio::spawn(async move {
                JavaProbe::probe(&executable).await.ok()
            }));
        }

        let mut installations = Vec::new();
        for handle in handles {
            if let Ok(Some(installation)) = handle.await {
                installations.push(installation);
            }
        }

        Ok(installations)
    }

    async fn detect_from_system(&self) -> Result<Vec<JavaInstallation>> {
        let mut installations = Vec::new();

        #[cfg(windows)]
        {
            if let Ok(registry_installations) = WindowsDetector::detect_from_registry().await {
                installations.extend(registry_installations);
            }
            if let Ok(system_installations) = WindowsDetector::detect_from_system().await {
                installations.extend(system_installations);
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(system_installations) = MacOSDetector::detect_from_system().await {
                installations.extend(system_installations);
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(system_installations) = LinuxDetector::detect_from_system().await {
                installations.extend(system_installations);
            }
        }

        Ok(installations)
    }

    fn get_canonical_path(&self, path: &Path) -> PathBuf {
        fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
    }

    fn is_valid_installation(&self, installation: &JavaInstallation) -> bool {
        let path_str = installation.executable.to_string_lossy().to_lowercase();

        let skip_patterns = [
            "javapath_target_",
            "java8path_target_",
            "javatmp",
            "system32",
            "syswow64",
        ];

        if skip_patterns
            .iter()
            .any(|pattern| path_str.contains(pattern))
        {
            return false;
        }

        #[cfg(windows)]
        if !self.is_32bit_system()
            && let Some(arch) = &installation.arch
            && (arch != "x64" && arch != "aarch64")
        {
            return false;
        }

        true
    }

    #[cfg(windows)]
    fn is_32bit_system(&self) -> bool {
        std::mem::size_of::<usize>() == 4
    }

    #[cfg(not(windows))]
    fn is_32bit_system(&self) -> bool {
        false
    }
}

impl Default for JavaDetector {
    fn default() -> Self {
        Self::new()
    }
}
