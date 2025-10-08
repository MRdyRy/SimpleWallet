use domain::wallet::wallet::WalletStatus;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Request {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub status: Option<WalletStatus>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransferRequest {
    pub from_id: i32,
    pub to_id: i32,
    pub amount: f64,
}
