use serde::{Deserialize, Serialize};
use crate::usecase::wallet::Usecase;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub usecase: Usecase
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AppConfig {
    pub db_host : String,
    pub db_user : String,
    pub db_pwd : String,
    pub log_format: String,
    pub log_level : String,
    pub user_host : String,
    pub transfer_host : String,
    pub http_timeout_seconds : u64,
    pub http_client_retry_attempts : u32,
}