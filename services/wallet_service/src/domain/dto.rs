use domain::wallet::wallet::WalletStatus;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Request {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub status: Option<WalletStatus>,
}
