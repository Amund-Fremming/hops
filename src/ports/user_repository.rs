use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{error::ServerError, user::User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(&self, id: Uuid) -> Result<Option<User>, ServerError>;
    async fn get_all_users(&self) -> Result<Vec<User>, ServerError>;
    async fn create_user(&self, user: User) -> Result<User, ServerError>;
    async fn update_user(&self, user: User) -> Result<User, ServerError>;
    async fn delete_user(&self, id: Uuid) -> Result<bool, ServerError>;
}
