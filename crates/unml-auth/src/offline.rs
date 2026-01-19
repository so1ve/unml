use async_trait::async_trait;
use unml_core::{Account, AccountType, AuthProvider, Credentials};
use uuid::Uuid;

use crate::{Error, Result};

pub struct OfflineAuthProvider;

impl OfflineAuthProvider {
    pub fn new() -> Self {
        Self
    }

    fn offline_uuid(username: &str) -> String {
        let name = format!("OfflinePlayer:{username}");
        Uuid::new_v3(&Uuid::NAMESPACE_DNS, name.as_bytes()).to_string()
    }
}

#[async_trait]
impl AuthProvider for OfflineAuthProvider {
    type Error = Error;

    async fn login(&self, credentials: Credentials) -> Result<Account> {
        match credentials {
            Credentials::Offline { username } => {
                let uuid = Self::offline_uuid(&username);
                Ok(Account {
                    username,
                    uuid,
                    access_token: String::new(),
                    refresh_token: None,
                    account_type: AccountType::Offline,
                })
            }
            _ => Err(Error::AuthFailed(
                "Offline provider only supports offline credentials".to_owned(),
            )),
        }
    }

    async fn refresh(&self, account: &Account) -> Result<Account> {
        Ok(account.clone())
    }

    async fn validate(&self, _account: &Account) -> Result<bool> {
        Ok(true)
    }
}

impl Default for OfflineAuthProvider {
    fn default() -> Self {
        Self::new()
    }
}
