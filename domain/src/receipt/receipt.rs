use serde::{Deserialize, Serialize};
use crate::base::base::AuditMetadata;
use crate::transfer::transfer::TransferStatus;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Receipt {
    pub id: Option<i32>,
    pub transaction_id: String,
    pub user_email: String,
    pub amount: f64,
    pub status: TransferStatus,
    pub execution_time: String,
    pub audit: AuditMetadata

}