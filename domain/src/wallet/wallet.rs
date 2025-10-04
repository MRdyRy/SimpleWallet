use crate::base::base::{AuditMetadata, Auditable};
use serde::{Deserialize, Serialize};
use crate::transfer::transfer::Transfer;
use crate::wallet::error::WalletError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum WalletStatus {
    Active,
    NonActive,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub id: Option<i32>,
    pub norek: String,
    pub user_id: i32,
    pub balance: f64,
    pub status: WalletStatus,
    pub audit: AuditMetadata,
}
impl Wallet {
    pub fn new(id: Option<i32>, norek: String, user_id: i32, inital_balance: f64, audit: AuditMetadata) -> Self {
        Self {
            id,
            norek,
            user_id,
            balance: inital_balance,
            status: WalletStatus::Active,
            audit
        }
    }

    pub fn credit (&mut self, amount: f64) -> Result<(), WalletError> {
        if amount <= 0.0 {
            return Err(WalletError::InvalidAmount(amount));
        }
        self.balance += amount;
        self.audit.touch();
        Ok(())
    }

    pub fn debit(&mut self, amount: f64) -> Result<(), WalletError> {
        if amount <= 0.0 {
            return Err(WalletError::InvalidAmount(amount));
        }

        if self.balance < amount {
            return Err(WalletError::InsufficientBalance(amount, self.balance));
        }

        self.balance -= amount;
        self.audit.touch();
        Ok(())
    }
}

impl Auditable for Wallet {
    fn audit(&self) -> &AuditMetadata { &self.audit }
    fn audit_mut(&mut self) -> &mut AuditMetadata { &mut self.audit }
}