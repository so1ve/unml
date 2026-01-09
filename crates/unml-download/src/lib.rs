mod error;
mod http;
mod mirror;
mod mojang;

use std::sync::OnceLock;

pub use error::{Error, Result};
pub use mirror::BMCLAPIDownloadProvider;
pub use mojang::MojangDownloadProvider;
use reqwest::Client;

static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

pub(crate) fn http_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent("UNML/0.1.0")
            .tcp_nodelay(true)
            .pool_max_idle_per_host(10)
            .build()
            .expect("Failed to create HTTP client")
    })
}
