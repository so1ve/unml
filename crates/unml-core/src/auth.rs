use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// 认证提供者
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// 登录
    async fn login(&self, credentials: Credentials) -> crate::Result<Account>;

    /// 刷新令牌
    async fn refresh(&self, account: &Account) -> crate::Result<Account>;

    /// 验证账号
    async fn validate(&self, account: &Account) -> crate::Result<bool>;
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
    pub account_type: AccountType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    Offline,
    Microsoft,
}
