use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::UnmlError;

/// 认证提供者
#[async_trait]
pub trait AuthProvider: Send + Sync {
    type Error: UnmlError;

    /// 登录
    async fn login(&self, credentials: Credentials) -> Result<Account, Self::Error>;

    /// 刷新令牌
    async fn refresh(&self, account: &Account) -> Result<Account, Self::Error>;

    /// 验证账号
    async fn validate(&self, account: &Account) -> Result<bool, Self::Error>;
}

/// 登录凭据
#[derive(Debug, Clone)]
pub enum Credentials {
    Offline { username: String },
    Microsoft { code: String },
}

/// 账号信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub uuid: String,
    pub access_token: String,
    /// 刷新令牌（仅微软账号需要）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    pub account_type: AccountType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    Offline,
    Microsoft,
}
