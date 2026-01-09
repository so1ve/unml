use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// 公开的 Azure 应用 Client ID（用于个人账户）
pub const CLIENT_ID: &str = "00000000402b5328";

/// Device Code 响应
#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u32,
    pub interval: u32,
}

/// Token 响应
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u32,
}

/// Token 错误响应
#[derive(Debug, Deserialize)]
struct TokenErrorResponse {
    error: String,
}

/// Xbox Live 认证响应
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveResponse {
    pub token: String,
    pub display_claims: XboxDisplayClaims,
}

#[derive(Debug, Deserialize)]
pub struct XboxDisplayClaims {
    pub xui: Vec<XboxUserInfo>,
}

#[derive(Debug, Deserialize)]
pub struct XboxUserInfo {
    pub uhs: String,
}

/// Minecraft 认证响应
#[derive(Debug, Deserialize)]
pub struct MinecraftAuthResponse {
    pub access_token: String,
    pub expires_in: u32,
}

/// Minecraft Profile 响应
#[derive(Debug, Deserialize)]
pub struct MinecraftProfile {
    pub id: String,
    pub name: String,
}

/// 请求设备码
pub async fn request_device_code(client: &reqwest::Client) -> Result<DeviceCodeResponse> {
    let response = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
        .form(&[
            ("client_id", CLIENT_ID),
            ("scope", "XboxLive.signin offline_access"),
        ])
        .send()
        .await
        .map_err(|e| Error::AuthFailed(format!("Failed to request device code: {e}")))?;

    response
        .json()
        .await
        .map_err(|e| Error::AuthFailed(format!("Failed to parse device code response: {e}")))
}

/// 轮询等待用户授权
pub async fn poll_for_token(
    client: &reqwest::Client,
    device_code: &str,
    interval: u32,
) -> Result<TokenResponse> {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(u64::from(interval))).await;

        let response = client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ("client_id", CLIENT_ID),
                ("device_code", device_code),
            ])
            .send()
            .await
            .map_err(|e| Error::AuthFailed(format!("Failed to poll token: {e}")))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| Error::AuthFailed(format!("Failed to read response: {e}")))?;

        if status.is_success() {
            return serde_json::from_str(&body)
                .map_err(|e| Error::AuthFailed(format!("Failed to parse token: {e}")));
        }

        let error: TokenErrorResponse = serde_json::from_str(&body)
            .map_err(|e| Error::AuthFailed(format!("Failed to parse error: {e}")))?;

        match error.error.as_str() {
            "authorization_pending" => continue,
            "slow_down" => {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
            "expired_token" => return Err(Error::TokenExpired),
            _ => return Err(Error::AuthFailed(format!("Auth error: {}", error.error))),
        }
    }
}

/// 用刷新令牌获取新的访问令牌
pub async fn refresh_token(client: &reqwest::Client, refresh_token: &str) -> Result<TokenResponse> {
    let response = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form(&[
            ("grant_type", "refresh_token"),
            ("client_id", CLIENT_ID),
            ("refresh_token", refresh_token),
        ])
        .send()
        .await
        .map_err(|e| Error::AuthFailed(format!("Failed to refresh token: {e}")))?;

    if !response.status().is_success() {
        return Err(Error::TokenExpired);
    }

    response
        .json()
        .await
        .map_err(|e| Error::AuthFailed(format!("Failed to parse token: {e}")))
}

/// Xbox Live 认证
pub async fn authenticate_xbox_live(
    client: &reqwest::Client,
    ms_access_token: &str,
) -> Result<XboxLiveResponse> {
    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct XboxAuthRequest<'a> {
        relying_party: &'a str,
        token_type: &'a str,
        properties: XboxAuthProperties<'a>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct XboxAuthProperties<'a> {
        auth_method: &'a str,
        site_name: &'a str,
        rps_ticket: String,
    }

    let request = XboxAuthRequest {
        relying_party: "http://auth.xboxlive.com",
        token_type: "JWT",
        properties: XboxAuthProperties {
            auth_method: "RPS",
            site_name: "user.auth.xboxlive.com",
            rps_ticket: format!("d={ms_access_token}"),
        },
    };

    let response = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&request)
        .send()
        .await
        .map_err(|e| Error::AuthFailed(format!("Xbox Live auth failed: {e}")))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(Error::AuthFailed(format!("Xbox Live auth failed: {body}")));
    }

    response
        .json()
        .await
        .map_err(|e| Error::AuthFailed(format!("Failed to parse Xbox Live response: {e}")))
}

/// XSTS 认证
pub async fn authenticate_xsts(
    client: &reqwest::Client,
    xbox_token: &str,
) -> Result<XboxLiveResponse> {
    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct XstsRequest<'a> {
        relying_party: &'a str,
        token_type: &'a str,
        properties: XstsProperties<'a>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct XstsProperties<'a> {
        sandbox_id: &'a str,
        user_tokens: Vec<&'a str>,
    }

    let request = XstsRequest {
        relying_party: "rp://api.minecraftservices.com/",
        token_type: "JWT",
        properties: XstsProperties {
            sandbox_id: "RETAIL",
            user_tokens: vec![xbox_token],
        },
    };

    let response = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .json(&request)
        .send()
        .await
        .map_err(|e| Error::AuthFailed(format!("XSTS auth failed: {e}")))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(Error::AuthFailed(format!("XSTS auth failed: {body}")));
    }

    response
        .json()
        .await
        .map_err(|e| Error::AuthFailed(format!("Failed to parse XSTS response: {e}")))
}

/// Minecraft 认证
pub async fn authenticate_minecraft(
    client: &reqwest::Client,
    user_hash: &str,
    xsts_token: &str,
) -> Result<MinecraftAuthResponse> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct MinecraftAuthRequest {
        identity_token: String,
    }

    let request = MinecraftAuthRequest {
        identity_token: format!("XBL3.0 x={user_hash};{xsts_token}"),
    };

    let response = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&request)
        .send()
        .await
        .map_err(|e| Error::AuthFailed(format!("Minecraft auth failed: {e}")))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(Error::AuthFailed(format!("Minecraft auth failed: {body}")));
    }

    response
        .json()
        .await
        .map_err(|e| Error::AuthFailed(format!("Failed to parse Minecraft auth response: {e}")))
}

/// 获取 Minecraft Profile
pub async fn get_minecraft_profile(
    client: &reqwest::Client,
    mc_access_token: &str,
) -> Result<MinecraftProfile> {
    let response = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .bearer_auth(mc_access_token)
        .send()
        .await
        .map_err(|e| Error::AuthFailed(format!("Failed to get profile: {e}")))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(Error::AuthFailed(format!(
            "Failed to get Minecraft profile: {body}"
        )));
    }

    response
        .json()
        .await
        .map_err(|e| Error::AuthFailed(format!("Failed to parse profile: {e}")))
}
