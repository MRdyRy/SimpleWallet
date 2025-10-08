use crate::domain::dto::Request;
use crate::repository::db::postgres::{WalletProvider, WalletRepository};
use anyhow::Result;
use domain::base::base::AuditMetadata;
use domain::wallet::wallet::Wallet as WalletDomain;

#[derive(Clone)]
pub struct Usecase {
    repo: WalletRepository,
}

pub trait Wallet {
    async fn get_or_create_wallet(&self, user_id: i32) -> Result<WalletDomain>;
    async fn transfer_balance(&self, from_id: i32, to_id: i32, amount: f64)
    -> Result<WalletDomain>;
    async fn delete_wallet(&self, id: i32) -> Result<()>;
    async fn update_balance(&self, user_id: i32, amount: f64) -> Result<WalletDomain>;
}

impl Usecase {
    pub fn new(repo: WalletRepository) -> Self {
        Self { repo }
    }

    fn construct_wallet(data: WalletDomain) -> WalletDomain {
        WalletDomain::new(
            data.id,
            data.norek,
            data.user_id,
            data.balance,
            AuditMetadata {
                created_date: data.audit.created_date,
                updated_date: data.audit.updated_date,
            },
        )
    }
}

impl Wallet for Usecase {
    async fn get_or_create_wallet(&self, user_id: i32) -> Result<WalletDomain> {
        tracing::info!("getting wallet {}", user_id);
        let opt_wallet = self.repo.get_wallet_by_userid(user_id).await?;
        match opt_wallet {
            None => {
                let create_wallet = self.repo.create_wallet(user_id, 0f64);
                match create_wallet.await {
                    Ok(d) => {
                        tracing::info!("created wallet for user_id {}", user_id);
                        Ok(Self::construct_wallet(d))
                    }
                    Err(e) => Err(anyhow::anyhow!("Failed to get or create wallet: {}", e)),
                }
            }
            Some(data) => {
                tracing::info!("wallet for user_id {} is exits", user_id);
                Ok(Self::construct_wallet(data))
            }
        }
    }

    async fn transfer_balance(
        &self,
        from_id: i32,
        to_id: i32,
        amount: f64,
    ) -> Result<WalletDomain> {
        tracing::info!("transfer balance wallet for user_id {}", from_id);
        if amount <= 0f64 {
            return Err(anyhow::anyhow!("Invalid transfer amount"));
        }

        let sender_wallet = self.repo.get_wallet_by_userid(from_id).await?;
        match sender_wallet {
            None => Err(anyhow::anyhow!("Sender wallet not found")),
            Some(mut sender_wallet) => match sender_wallet.debit(amount) {
                Ok(_) => {
                    let receiver_wallet = self.repo.get_wallet_by_userid(to_id).await?;
                    match receiver_wallet {
                        None => Err(anyhow::anyhow!("Receiver wallet not found")),
                        Some(_receiver_wallet) => {
                            tracing::info!(
                                "sender and receiver wallet are exists, processing transfer balance....."
                            );

                            let (sender_balance, receiver_balance) =
                                self.repo.transfer_balance(from_id, to_id, amount).await?;

                            tracing::info!(
                                "transfer done, current balance sender: {} receiver: {}",
                                sender_balance,
                                receiver_balance
                            );

                            Ok(sender_wallet)
                        }
                    }
                }
                Err(e) => Err(anyhow::anyhow!(e)),
            },
        }
    }

    async fn delete_wallet(&self, id: i32) -> Result<()> {
        tracing::info!("updating wallet for user_id {}", id);
        let opt_wallet = self.repo.get_wallet_by_userid(id).await?;
        match opt_wallet {
            None => Err(anyhow::anyhow!("Wallet not found")),
            Some(wallet) => {
                self.repo.delete_wallet(id).await?;
                Ok(())
            }
        }
    }

    async fn update_balance(&self, user_id: i32, amount: f64) -> Result<WalletDomain> {
        tracing::info!("updating wallet for user_id {}", user_id);
        let opt_wallet = self.repo.get_wallet_by_userid(user_id).await?;
        match opt_wallet {
            None => Err(anyhow::anyhow!("Wallet not found")),
            Some(mut wallet) => {
                let _ = self.repo.update_balance(user_id, amount).await;
                wallet.debit(amount)?;
                Ok(wallet)
            }
        }
    }
}
