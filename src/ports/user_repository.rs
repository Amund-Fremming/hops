use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::User;

#[derive(Debug)]
pub enum UserRepoError {
    NotFound,
    Conflict,
    DatabaseError(String),
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(&self, id: Uuid) -> Result<Option<User>, UserRepoError>;
    async fn get_all_users(&self) -> Result<Vec<User>, UserRepoError>;
    async fn create_user(&self, user: User) -> Result<User, UserRepoError>;
    async fn update_user(&self, user: User) -> Result<User, UserRepoError>;
    async fn delete_user(&self, id: Uuid) -> Result<bool, UserRepoError>;
}
