use async_trait::async_trait;

use crate::model::users::{User, UserId};

#[async_trait]
pub trait UserRepository {
    async fn get(&self, id: &UserId) -> anyhow::Result<Option<User>>;
}
