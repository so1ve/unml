use async_trait::async_trait;
use unml_core::{Account, AccountType, AuthProvider, Credentials};

pub struct OfflineAuthProvider;

impl OfflineAuthProvider {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]

impl AuthProvider for OfflineAuthProvider {
    async fn login(&self, credentials: Credentials) -> unml_core::Result<Account> {
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
            _ => Err(unml_core::Error::AuthFailed(
                "Offline provider only supports offline credentials".to_string(),
            )),
        }
    }

    async fn refresh(&self, account: &Account) -> unml_core::Result<Account> {
        Ok(account.clone())
    }

    async fn validate(&self, _account: &Account) -> unml_core::Result<bool> {
        Ok(true)
    }
}

impl Default for OfflineAuthProvider {
    fn default() -> Self {
        Self::new()
    }
}
