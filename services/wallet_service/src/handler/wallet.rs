use crate::app::AppState;
use crate::domain::dto::TransferRequest;
use crate::usecase::wallet::Wallet;
use axum::extract::{Path, State};
use axum::Json;
use domain::base::base::BaseResponse;
use domain::wallet::wallet::Wallet as WalletDomain;
use reqwest::StatusCode;

/// Transfer between 2 wallets
/// request :
///   - sender id
///   - receiver id
///   - amount
///
/// Validates the sender's balance.
///
/// If the sender's balance is less than the transaction amount:
/// - Returns an "Insufficient balance" message.
///
/// Otherwise:
/// - Proceeds with the transaction.
/// - Deducts the specified amount from the sender's balance.
/// - Adds the specified amount to the receiver's balance.
pub async fn transfer_wallet(
    State(state): State<AppState>,
    Json(request): Json<TransferRequest>,
) -> (StatusCode, Json<BaseResponse<WalletDomain>>) {
    tracing::info!("inquiry wallet for request: {:?}", request);
    match state
        .usecase
        .transfer_balance(request.from_id, request.to_id, request.amount)
        .await
    {
        Ok(data) => {
            let response: BaseResponse<WalletDomain> =
                BaseResponse::new("".to_string(), "Success".to_string(), Some(data));
            (StatusCode::OK, Json::from(response))
        }
        Err(e) => {
            let response = BaseResponse::new("".to_string(), format!("{e}"), None);
            (StatusCode::INTERNAL_SERVER_ERROR, Json::from(response))
        }
    }
}

/// Retrieves the wallet by its ID.
///
/// If the wallet does not exist, a new wallet is created with a balance of 0.
pub async fn get_wallet_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<BaseResponse<WalletDomain>>) {
    tracing::info!("inquiry wallet for id: {:?}", id);
    match state.usecase.get_or_create_wallet(id).await {
        Ok(data) => {
            let response = BaseResponse::new("".to_string(), "Success".to_string(), Some(data));
            (StatusCode::OK, Json::from(response))
        }
        Err(e) => {
            let response = BaseResponse::new("".to_string(), format!("{e}"), None);
            (StatusCode::INTERNAL_SERVER_ERROR, Json::from(response))
        }
    }
}

/// Deletes the wallet by its ID.
/// If the wallet exists, its status is marked as inactive.
pub async fn delete_wallet(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<BaseResponse<bool>>) {
    tracing::info!("delete wallet for id: {:?}", id);
    match state.usecase.delete_wallet(id).await {
        Ok(_) => (
            StatusCode::OK,
            Json::from(BaseResponse::new(
                "".to_string(),
                "Success".to_string(),
                Some(true),
            )),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json::from(BaseResponse::new(
                "".to_string(),
                format!("{e}"),
                Some(false),
            )),
        ),
    }
}
