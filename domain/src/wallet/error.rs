use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Insufficient balance: attempted to debit {0}, but only {1} available")]
    InsufficientBalance(f64, f64),
    #[error("Invalid amount: {0}")]
    InvalidAmount(f64),
}