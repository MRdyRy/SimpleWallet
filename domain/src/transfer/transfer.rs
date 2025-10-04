use crate::base::base::{AuditMetadata, Auditable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransferStatus {
    Pending,
    Success,
    Failed,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transfer {
    pub id: Option<i32>,
    pub account_debet: String,
    pub account_credit: String,
    pub amount: f64,
    pub status: TransferStatus,
    pub audit: AuditMetadata,
}

impl Transfer {
    pub fn new(
        account_debet: &str,
        account_credit: &str,
        amount: f64,
        status: TransferStatus,
        audit: AuditMetadata,
    ) -> Self {
        Self {
            id: None,
            account_debet: account_debet.to_string(),
            account_credit: account_credit.to_string(),
            amount,
            status,
            audit,
        }
    }

    pub fn mark_success(&mut self) {
        self.status = TransferStatus::Success;
        self.audit.touch();
    }

    pub fn mark_failed(&mut self) {
        self.status = TransferStatus::Failed;
        self.audit.touch();
    }

    pub fn mark_pending(&mut self) {
        self.status = TransferStatus::Pending;
        self.audit.touch();
    }
}
impl Auditable for Transfer {
    fn audit(&self) -> &AuditMetadata { &self.audit }
    fn audit_mut(&mut self) -> &mut AuditMetadata { &mut self.audit }
}