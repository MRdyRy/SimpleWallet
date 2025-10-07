use crate::domain::dto::Request;
use crate::repository::db::postgres::WalletRepository;
use anyhow::Result;
use domain::wallet::wallet::Wallet as WalletDomain;

#[derive(Clone)]
pub struct Usecase {
    repo: WalletRepository,
}

pub trait Wallet {
    async fn get_or_create_wallet(&self, user_id: i32) -> Result<WalletDomain>;
    async fn transfer_balance(&self, from_id: i32, to_id: i32, amount: u64)
    -> Result<WalletDomain>;
    async fn update_wallet(&self, id: i32, request: Request) -> Result<WalletDomain>;
    async fn update_balance(&self, user_id: i32, amount: u64) -> Result<WalletDomain>;
}
