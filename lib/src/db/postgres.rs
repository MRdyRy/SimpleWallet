use deadpool_postgres::{Config, Pool, Runtime};
use once_cell::sync::OnceCell;
use tokio_postgres::NoTls;
use tracing::info;

static POOL: OnceCell<Pool> = OnceCell::new();

pub fn init_pool() -> &'static Pool {
    let _ = dotenvy::dotenv();
    POOL.get_or_init(|| {
        let mut cfg = Config::new();
        cfg.host = std::env::var("DB_HOST").ok();
        cfg.port = std::env::var("DB_PORT").ok().and_then(|p| p.parse().ok());
        cfg.dbname = std::env::var("DB_NAME").ok();
        cfg.password = std::env::var("DB_PASSWORD").ok();

        if let Ok(max_pool) = std::env::var("DB_MAX_POOL").and_then(|v| {
            v.parse::<usize>()
                .map_err(|_| std::env::VarError::NotPresent)
        }) {
            cfg.pool = Some(deadpool_postgres::PoolConfig {
                max_size: max_pool,
                ..Default::default()
            });
        }

        let pool = cfg
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("create db pool");

        info!("db pool created");
        pool
    })
}
