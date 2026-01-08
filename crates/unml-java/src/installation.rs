use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Java 安装信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaInstallation {
    /// Java 可执行文件路径
    pub executable: PathBuf,

    /// 完整版本字符串（如 "1.8.0_292" 或 "17.0.1"）
    pub version: String,

    /// 主版本号（如 8, 11, 17）
    pub major_version: u32,

    /// Java 供应商（如 "Oracle", "OpenJDK", "Azul Zulu"）
    pub vendor: Option<String>,

    /// JAVA_HOME 路径
    pub home: PathBuf,

    /// 架构（如 "x64", "aarch64"）
    pub arch: Option<String>,
}

impl JavaInstallation {
    /// 检查是否满足版本要求
    pub fn satisfies(&self, required: crate::JavaVersion) -> bool {
        self.major_version >= required.major
    }

    /// 获取 Java 可执行文件名
    pub fn executable_name() -> &'static str {
        if cfg!(windows) { "java.exe" } else { "java" }
    }

    /// 获取 javac 可执行文件路径
    pub fn javac_path(&self) -> PathBuf {
        let name = if cfg!(windows) { "javac.exe" } else { "javac" };
        self.home.join("bin").join(name)
    }
}
