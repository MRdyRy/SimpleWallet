use crate::app::AppState;
use crate::handler::wallet::{delete_wallet, get_wallet_by_id, transfer_wallet};
use axum::routing::{get, post};
use axum::Router;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/wallet/transfer", post(transfer_wallet))
        .route("/wallet/delete/:id", get(delete_wallet))
        .route("/wallet/inquiry/:id", get(get_wallet_by_id))
        .with_state(app_state)
}