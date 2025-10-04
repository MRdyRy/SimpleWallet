use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub base_url: HashMap<String, String>,
    pub timeout_seconds: u64,
    pub max_idle_connections: usize,
    pub pool_idle_timeout_seconds: u64,
    pub default_header_auth: Option<String>
}

impl HttpClientConfig {
    pub fn from_env ()-> Self {
        dotenvy::dotenv().ok();

        let timeout_seconds = std::env::var("TIMEOUT_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        let max_idle_connections = std::env::var("MAX_IDLE_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        let pool_idle_timeout_seconds = std::env::var("POOL_IDLE_TIMEOUT_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);
        let default_header_auth = std::env::var("DEFAULT_HEADER_AUTH");

        // Load multiple base URLs (e.g., USER_SERVICE_URL, TRANSFER_SERVICE_URL)
        let mut base_urls = HashMap::new();
        for (key, value) in std::env::vars() {
            if key.ends_with("_SERVICE_URL") {
                base_urls.insert(key.to_lowercase(), value);
            }
        }

        Self {
            base_url: base_urls,
            timeout_seconds,
            max_idle_connections,
            pool_idle_timeout_seconds,
            default_header_auth: None,
        }
    }

    pub fn resolve_base_url (&self, key: &str) -> Option<String> {
        let key = format!("{}_service_url",key.to_lowercase());
        self.base_url.get(&key).cloned()
    }
}