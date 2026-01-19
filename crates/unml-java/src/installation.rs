use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::JavaVersion;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaInstallation {
    pub executable: PathBuf,
    pub version: String,
    pub major_version: u32,
    pub vendor: Option<String>,
    pub home: PathBuf,
    pub arch: Option<String>,
}

impl JavaInstallation {
    pub fn satisfies(&self, required: JavaVersion) -> bool {
        self.major_version >= required.major
    }

    pub fn executable_name() -> &'static str {
        if cfg!(windows) { "java.exe" } else { "java" }
    }

    pub fn javac_path(&self) -> PathBuf {
        let name = if cfg!(windows) { "javac.exe" } else { "javac" };

        let from_home = self.home.join("bin").join(name);
        if fs::metadata(&from_home).is_ok() {
            return from_home;
        }

        // maybe symbolic link
        if let Some(bin_dir) = self.executable.parent() {
            return bin_dir.join(name);
        }

        from_home
    }
}
