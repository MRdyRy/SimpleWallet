use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(String),

    #[error("Deserialization failed: {0}")]
    DeserializeFailed(String),

    #[error("Unexpected status code: {0}")]
    UnexpectedStatus(u16),
}