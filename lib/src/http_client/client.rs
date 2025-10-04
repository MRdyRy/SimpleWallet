use anyhow::Result;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use tracing::{error, info};

use crate::http_client::config::HttpClientConfig;
use crate::http_client::error::HttpClientError;
static CONFIG: OnceCell<HttpClientConfig> = OnceCell::new();

lazy_static! {
    static ref HTTP_CLIENT: Client = {
        let cfg = CONFIG.get_or_init(HttpClientConfig::from_env);
        info!(
            "Initializing global HTTP client with multi-service config: {:?}",
            cfg.base_url
        );

        let mut builder = Client::builder()
            .timeout(Duration::from_secs(cfg.timeout_seconds))
            .pool_idle_timeout(Duration::from_secs(cfg.pool_idle_timeout_seconds))
            .pool_max_idle_per_host(cfg.max_idle_connections);

        if let Some(ref auth_header) = cfg.default_header_auth {
            builder = builder.default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(auth_header).unwrap(),
                );
                headers
            });
        }

        builder.build().expect("Failed to build global HTTP client")
    };
}

pub fn init_http_client() {
    let _ = CONFIG.set(HttpClientConfig::from_env());
    info!("✅ HTTP client configuration loaded");
}

pub fn client() -> &'static Client {
    &HTTP_CLIENT
}

/// GET JSON from full or relative URL
pub async fn get_json<T: DeserializeOwned>(service: &str, path: &str) -> Result<T> {
    let full_url = resolve_url(service, path);
    info!("GET {}", full_url);
    let res = client()
        .get(&full_url)
        .send()
        .await
        .map_err(|e| HttpClientError::RequestFailed(e.to_string()))?;
    handle_response(res).await
}

/// POST JSON
pub async fn post_json<T: DeserializeOwned, B: Serialize>(
    service: &str,
    path: &str,
    body: &B,
) -> Result<T> {
    let full_url = resolve_url(service, path);
    info!("POST {}", full_url);
    let res = client()
        .post(&full_url)
        .json(body)
        .send()
        .await
        .map_err(|e| HttpClientError::RequestFailed(e.to_string()))?;
    handle_response(res).await
}

/// PUT JSON
pub async fn put_json<T: DeserializeOwned, B: Serialize>(
    service: &str,
    path: &str,
    body: &B,
) -> Result<T> {
    let full_url = resolve_url(service, path);
    info!("PUT {}", full_url);
    let res = client()
        .put(&full_url)
        .json(body)
        .send()
        .await
        .map_err(|e| HttpClientError::RequestFailed(e.to_string()))?;
    handle_response(res).await
}

/// DELETE
pub async fn delete(service: &str, path: &str) -> Result<()> {
    let full_url = resolve_url(service, path);
    info!("DELETE {}", full_url);
    let res = client()
        .delete(&full_url)
        .send()
        .await
        .map_err(|e| HttpClientError::RequestFailed(e.to_string()))?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Delete failed with status {}",
            res.status().as_u16()
        ))
    }
}

/// Helper to resolve service base URL dynamically
fn resolve_url(service: &str, endpoint: &str) -> String {
    let cfg = CONFIG.get_or_init(HttpClientConfig::from_env);
    if endpoint.starts_with("http") {
        endpoint.to_string()
    } else if let Some(base) = cfg.resolve_base_url(service) {
        format!(
            "{}/{}",
            base.trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        )
    } else {
        endpoint.to_string() // fallback
    }
}

/// Deserialize and error-check JSON response
async fn handle_response<T: DeserializeOwned>(res: Response) -> Result<T> {
    let status = res.status();
    let text = res.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<T>(&text)
            .map_err(|e| HttpClientError::DeserializeFailed(e.to_string()).into())
    } else {
        error!("HTTP error {}: {}", status.as_u16(), text);
        Err(HttpClientError::UnexpectedStatus(status.as_u16()).into())
    }
}
