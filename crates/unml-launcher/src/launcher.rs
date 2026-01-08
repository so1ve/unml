use std::path::PathBuf;

use async_trait::async_trait;
use unml_core::{Account, GameLauncher, GameProcess, LaunchConfig};

pub struct StandardLauncher {
    #[allow(dead_code)]
    game_dir: PathBuf,
}

impl StandardLauncher {
    pub fn new() -> Self {
        Self {
            game_dir: PathBuf::from("./minecraft"),
        }
    }
}

#[async_trait]
impl GameLauncher for StandardLauncher {
    async fn launch(
        &self,
        _version: &str,
        _account: &Account,
        _config: LaunchConfig,
    ) -> unml_core::Result<GameProcess> {
        // TODO: 实现
        Ok(GameProcess::new(None))
    }
}

impl Default for StandardLauncher {
    fn default() -> Self {
        Self::new()
    }
}
