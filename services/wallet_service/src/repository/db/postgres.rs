use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::Utc;
use deadpool_postgres::GenericClient;
use domain::base::base::AuditMetadata;
use domain::wallet::wallet::{Wallet, WalletStatus};
use mockall::automock;

#[derive(Debug, Clone)]
pub struct WalletRepository {
    pool: deadpool_postgres::Pool,
}

#[async_trait]
pub trait WalletProvider {
    async fn get_wallet_by_userid(&self, user_id: i32) -> Result<Option<Wallet>>;
    async fn create_wallet(&self, user_id: i32, balance: f64) -> Result<Wallet>;
    async fn update_balance(&self, user_id: i32, upcoming_balance: f64) -> Result<()>;
    async fn transfer_balance(&self, from_id: i32, to_id: i32, amount: f64) -> Result<(f64, f64)>;
    async fn delete_wallet(&self, id: i32) -> Result<()>;
}

#[async_trait]
pub trait DbProvider: WalletProvider + Send + Sync {}

#[async_trait]
impl<T: WalletProvider + Send + Sync> DbProvider for T {}

impl WalletRepository {
    pub fn new(pool: deadpool_postgres::Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
#[automock]
impl WalletProvider for WalletRepository {
    async fn get_wallet_by_userid(&self, user_id: i32) -> Result<Option<Wallet>> {
        tracing::info!("get wallet by user id {:?}", user_id);
        let client = self.pool.get().await?;
        let result_opt = client.query_opt(
            "SELECT id, norek, user_id, balance, status, created_date, updated_date from WALLET_DIGITAL.DATA_WALLET WHERE user_id = $1", &[&user_id])
            .await?;
        Ok(result_opt.map(|row| Wallet {
            id: Some(row.get("id")),
            norek: row.get("norek"),
            user_id,
            balance: row.get("ballance"),
            status: row.get("status"),
            audit: AuditMetadata {
                created_date: row.get("created_date"),
                updated_date: Some(row.get("updated_date")),
            },
        }))
    }

    async fn create_wallet(&self, user_id: i32, balance: f64) -> Result<Wallet> {
        tracing::info!("create wallet for user id : {:?}", user_id);

        let client = self.pool.get().await?;
        let now = Utc::now();
        let status = WalletStatus::Active;
        let norek = "";

        let result = client.query_one(
            "INSERT INTO WALLET_DIGITAL.DATA_WALLET (user_id, balance, norek, status, created_date, updated_date)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, user_id, balance, status, created_date, updated_date",
             &[&user_id, &balance, &norek, &status, &now, &now]
        ).await?;
        Ok(Wallet {
            id: Some(result.get("id")),
            norek: result.get("norek"),
            user_id: result.get("user_id"),
            balance: result.get("balance"),
            status: result.get("status"),
            audit: AuditMetadata {
                created_date: result.get("created_date"),
                updated_date: Some(result.get("updated_date")),
            },
        })
    }

    async fn update_balance(&self, user_id: i32, upcoming_balance: f64) -> Result<()> {
        tracing::info!(
            "update balance for user_id : {:?} with balance : {:?}",
            user_id,
            upcoming_balance
        );
        let client = self.pool.get().await?;
        let now = Utc::now();
        client
            .execute(
                "UPDATE WALLET_DIGITAL.DATA_WALLET
            SET balance = balance + $1
            updated_date = $2
            WHERE user_id = $3",
                &[&upcoming_balance, &now, &user_id],
            )
            .await?;
        Ok(())
    }

    async fn transfer_balance(&self, from_id: i32, to_id: i32, amount: f64) -> Result<(f64, f64)> {
        tracing::info!(
            "transfer balance from {:?} to {:?} with value {:?}",
            from_id,
            to_id,
            amount
        );

        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        let now = Utc::now();

        let sender_result = tx
            .query_opt(
                "UPDATE WALLET_DIGITAL.DATA_WALLET
            SET balance = balance - $1, updated_date = $2
            WHERE user_id = $3 AND balance >= $3
            RETURNING balance",
                &[&amount, &now, &from_id],
            )
            .await?;

        let sender_result = sender_result.ok_or_else(|| anyhow!("Insufficient balance"))?;
        let sender_new_balance: f64 = sender_result.get("balance");

        let receiver_result = tx
            .query_one(
                "UPDATE WALLET_DIGITAL.DATA_WALLET
            SET balance = balance + $1,
            updated_date = $2
            WHERE user_id = $3
            RETURNING balance",
                &[&amount, &now, &to_id],
            )
            .await?;

        let receiver_new_balance = receiver_result.get("balance");

        tx.commit().await?;

        Ok((sender_new_balance, receiver_new_balance))
    }

    async fn delete_wallet(&self, id: i32) -> Result<()> {
        tracing::info!("delete wallet for id : {:?}", id);

        let client = self.pool.get().await?;
        let status = WalletStatus::Inactive;
        client
            .execute(
                "UPDATE WALLET_DIGITAL.DATA_WALLET SET status = $1 WHERE user_id = $2",
                &[&status, &id],
            )
            .await?;
        Ok(())
    }
}
