use std::path::PathBuf;

use async_trait::async_trait;

use crate::UnmlError;

/// 游戏文件仓库管理
#[async_trait]
pub trait GameRepository: Send + Sync {
    type Error: UnmlError;

    /// 列出所有已安装的游戏版本
    async fn list_installed_versions(&self) -> Result<Vec<String>, Self::Error>;

    /// 检查版本是否完整（所有文件都存在且校验通过）
    async fn verify_version(&self, version_id: &str) -> Result<bool, Self::Error>;

    /// 获取版本的安装路径
    fn get_version_path(&self, version_id: &str) -> PathBuf;

    /// 获取版本 JSON 文件路径
    fn get_version_json(&self, version_id: &str) -> PathBuf;

    /// 获取版本 JAR 文件路径
    fn get_version_jar(&self, version_id: &str) -> PathBuf;
}

/// 游戏启动器
#[async_trait]
pub trait GameLauncher: Send + Sync {
    type Error: UnmlError;

    /// 启动游戏
    async fn launch(
        &self,
        version: &str,
        account: &crate::Account,
        config: LaunchConfig,
    ) -> Result<GameProcess, Self::Error>;
}

/// 启动配置
#[derive(Debug, Clone)]
pub struct LaunchConfig {
    pub java_path: PathBuf,
    pub jvm_args: Vec<String>,
    pub game_args: Vec<String>,
    pub window_width: u32,
    pub window_height: u32,
}

/// 游戏进程（占位符）
#[derive(Debug)]
pub struct GameProcess {
    pid: Option<u32>,
}

impl GameProcess {
    pub fn new(pid: Option<u32>) -> Self {
        Self { pid }
    }

    pub fn pid(&self) -> Option<u32> {
        self.pid
    }
}
