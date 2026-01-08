use async_trait::async_trait;
use unml_core::{Account, AuthProvider, Credentials};

pub struct MicrosoftAuthProvider;

impl MicrosoftAuthProvider {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]

impl AuthProvider for MicrosoftAuthProvider {
    async fn login(&self, _credentials: Credentials) -> unml_core::Result<Account> {
        // TODO: 实现微软登录
        todo!("Microsoft auth not implemented yet")
    }

    async fn refresh(&self, _account: &Account) -> unml_core::Result<Account> {
        // TODO: 实现
        todo!("refresh not implemented yet")
    }

    async fn validate(&self, _account: &Account) -> unml_core::Result<bool> {
        // TODO: 实现
        Ok(false)
    }
}

impl Default for MicrosoftAuthProvider {
    fn default() -> Self {
        Self::new()
    }
}
