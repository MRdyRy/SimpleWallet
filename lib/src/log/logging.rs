use once_cell::sync::OnceCell;
use tracing_subscriber::{fmt, EnvFilter};

static INIT: OnceCell<()> = OnceCell::new();
pub fn init(service_name: &str) {
    let _ = dotenvy::dotenv();
    INIT.get_or_init(|| {
        let log_format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "plain".to_string());
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        match log_format.as_str() {
            "json" => {
                fmt::Subscriber::builder()
                    .with_env_filter(env_filter)
                    .json()
                    .flatten_event(true)
                    .with_current_span(false)
                    .with_target(false)
                    .init();
            }
            _ => {
                fmt::Subscriber::builder()
                    .with_env_filter(env_filter)
                    .with_target(false)
                    .compact()
                    .init();
            }
        }
        tracing::info!("🚀 Service '{}' initialized!", service_name);
    });
}
