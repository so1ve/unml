use std::error::Error;

use thiserror::Error;

/// 所有 unml 错误类型的基础 trait
pub trait UnmlError: Error + Send + Sync + 'static {}
impl<T: Error + Send + Sync + 'static> UnmlError for T {}

/// 通用 IO 错误包装
#[derive(Debug, Error)]
#[error("IO error: {0}")]
pub struct IoError(#[from] pub std::io::Error);

/// 通用 HTTP 错误
#[derive(Debug, Error)]
#[error("HTTP error: {0}")]
pub struct HttpError(pub String);

/// 通用 JSON 解析错误
#[derive(Debug, Error)]
#[error("JSON error: {0}")]
pub struct JsonError(#[from] pub serde_json::Error);

/// 校验和不匹配错误
#[derive(Debug, Error)]
#[error("Checksum mismatch: expected {expected}, got {actual}")]
pub struct ChecksumError {
    pub expected: String,
    pub actual: String,
}
