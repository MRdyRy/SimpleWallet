use tracing_subscriber::{fmt, EnvFilter};
pub fn init(service_name: &str) {
    let env_filter = EnvFilter::from_default_env().add_directive("info".parse().unwrap());

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .json()
        .init();
    tracing::info!("🚀 Service '{}' initialized!", service_name);
}
