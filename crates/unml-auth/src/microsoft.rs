use std::sync::OnceLock;

use async_trait::async_trait;
use unml_core::{Account, AccountType, AuthProvider, Credentials};

use crate::oauth::{self, DeviceCodeResponse, TokenResponse};
use crate::{Error, Result};

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .user_agent("UNML/0.1.0")
            .build()
            .expect("Failed to create HTTP client")
    })
}

/// 微软登录回调，用于获取设备码信息
pub type DeviceCodeCallback = Box<dyn Fn(&DeviceCodeResponse) + Send + Sync>;

pub struct MicrosoftAuthProvider {
    device_code_callback: Option<DeviceCodeCallback>,
}

impl MicrosoftAuthProvider {
    pub fn new() -> Self {
        Self {
            device_code_callback: None,
        }
    }

    /// 设置设备码回调（用于通知用户访问 URL 并输入代码）
    pub fn with_device_code_callback(mut self, callback: DeviceCodeCallback) -> Self {
        self.device_code_callback = Some(callback);
        self
    }

    /// 用 MS Token 换取 Minecraft Account
    async fn exchange_token_for_account(&self, ms_token: &TokenResponse) -> Result<Account> {
        let client = http_client();

        // 1. Xbox Live 认证
        let xbox_response = oauth::authenticate_xbox_live(client, &ms_token.access_token).await?;
        let user_hash = xbox_response
            .display_claims
            .xui
            .first()
            .ok_or_else(|| Error::AuthFailed("No Xbox user info".to_string()))?
            .uhs
            .clone();

        // 2. XSTS 认证
        let xsts_response = oauth::authenticate_xsts(client, &xbox_response.token).await?;

        // 3. Minecraft 认证
        let mc_auth =
            oauth::authenticate_minecraft(client, &user_hash, &xsts_response.token).await?;

        // 4. 获取 Minecraft Profile
        let profile = oauth::get_minecraft_profile(client, &mc_auth.access_token).await?;

        Ok(Account {
            username: profile.name,
            uuid: profile.id,
            access_token: mc_auth.access_token,
            refresh_token: Some(ms_token.refresh_token.clone()),
            account_type: AccountType::Microsoft,
        })
    }

    /// 完整登录流程
    async fn full_login(&self) -> Result<Account> {
        let client = http_client();

        // 1. 获取设备码
        let device_code = oauth::request_device_code(client).await?;

        // 通知用户
        if let Some(ref callback) = self.device_code_callback {
            callback(&device_code);
        } else {
            // 默认打印到控制台
            eprintln!(
                "Please visit {} and enter code: {}",
                device_code.verification_uri, device_code.user_code
            );
        }

        // 2. 轮询等待用户授权
        let ms_token =
            oauth::poll_for_token(client, &device_code.device_code, device_code.interval).await?;

        // 3. 换取 Minecraft Account
        self.exchange_token_for_account(&ms_token).await
    }
}

#[async_trait]
impl AuthProvider for MicrosoftAuthProvider {
    type Error = Error;

    async fn login(&self, credentials: Credentials) -> Result<Account> {
        match credentials {
            Credentials::Microsoft { .. } => self.full_login().await,
            _ => Err(Error::AuthFailed(
                "Microsoft provider only supports Microsoft credentials".to_string(),
            )),
        }
    }

    async fn refresh(&self, account: &Account) -> Result<Account> {
        let refresh_token = account
            .refresh_token
            .as_ref()
            .ok_or_else(|| Error::AuthFailed("No refresh token available".to_string()))?;

        let client = http_client();

        // 用 refresh_token 获取新的 MS token
        let ms_token = oauth::refresh_token(client, refresh_token).await?;

        // 换取新的 Minecraft Account
        self.exchange_token_for_account(&ms_token).await
    }

    async fn validate(&self, account: &Account) -> Result<bool> {
        let client = http_client();

        // 尝试获取 profile 来验证 token 是否有效
        match oauth::get_minecraft_profile(client, &account.access_token).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

impl Default for MicrosoftAuthProvider {
    fn default() -> Self {
        Self::new()
    }
}
