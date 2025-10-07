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
const SERVICE_NAME: &'static str = "WALLET_SERVICE";
#[tokio::main]
async fn main() {
    init(SERVICE_NAME);
    tracing::info!("starting wallet service ...!")
}
