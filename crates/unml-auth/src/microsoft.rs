use async_trait::async_trait;
use unml_core::{Account, AuthProvider, Credentials};

use crate::{Error, Result};

pub struct MicrosoftAuthProvider;

impl MicrosoftAuthProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AuthProvider for MicrosoftAuthProvider {
    type Error = Error;

    async fn login(&self, _credentials: Credentials) -> Result<Account> {
        // TODO: 实现微软登录
        todo!("Microsoft auth not implemented yet")
    }

    async fn refresh(&self, _account: &Account) -> Result<Account> {
        // TODO: 实现
        todo!("refresh not implemented yet")
    }

    async fn validate(&self, _account: &Account) -> Result<bool> {
        // TODO: 实现
        Ok(false)
    }
}

impl Default for MicrosoftAuthProvider {
    fn default() -> Self {
        Self::new()
    }
}
