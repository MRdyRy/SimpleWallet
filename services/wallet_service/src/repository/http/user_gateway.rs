use anyhow::Result;
use async_trait::async_trait;
use domain::user::user::User;
use lib::http_client::client::get_json;
use mockall::automock;

#[derive(Debug, Clone)]
pub struct RestRepository;

#[async_trait]
pub trait UserProvider: Send + Sync {
    async fn find_user_by_id(user_id: i32) -> Result<User>;
}

#[automock]
#[async_trait]
impl UserProvider for RestRepository {
    async fn find_user_by_id(user_id: i32) -> Result<User> {
        let user: User = get_json("user", &format!("/users/{}", user_id)).await?;
        Ok(user)
    }
}
