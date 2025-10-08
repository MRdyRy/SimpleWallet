mod app;

use lib::db::postgres::init_pool;
use lib::http_client::client::init_http_client;
use lib::log::logging::init;

mod repository {
    pub mod db;
    pub mod http;
}

mod usecase {
    pub mod wallet;
}
mod domain {
    pub mod dto;
}

mod handler {
    pub mod health;
    pub mod router;
    pub mod wallet;
}

const SERVICE_NAME: &'static str = "WALLET_SERVICE";
#[tokio::main]
async fn main() {
    init(SERVICE_NAME);
    tracing::info!("starting wallet service ...!");
}
