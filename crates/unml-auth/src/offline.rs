use async_trait::async_trait;
use unml_core::{Account, AccountType, AuthProvider, Credentials};

use crate::{Error, Result};

pub struct OfflineAuthProvider;

impl OfflineAuthProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AuthProvider for OfflineAuthProvider {
    type Error = Error;

    async fn login(&self, credentials: Credentials) -> Result<Account> {
        match credentials {
            Credentials::Offline { username } => {
                let uuid = uuid::Uuid::new_v4().to_string();
                Ok(Account {
                    username,
                    uuid,
                    access_token: String::new(),
                    account_type: AccountType::Offline,
                })
            }
            _ => Err(Error::AuthFailed(
                "Offline provider only supports offline credentials".to_string(),
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
