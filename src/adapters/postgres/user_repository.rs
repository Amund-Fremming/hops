use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::{error::ServerError, user::User};
use crate::ports::user_repository::UserRepository;

pub struct PostgresUserRepository {
    pool: Pool<Postgres>,
}

impl PostgresUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn get_user(&self, _id: Uuid) -> Result<Option<User>, ServerError> {
        Ok(None)
    }

    async fn get_all_users(&self) -> Result<Vec<User>, ServerError> {
        todo!()
    }

    async fn create_user(&self, _user: User) -> Result<User, ServerError> {
        todo!()
    }

    async fn update_user(&self, _user: User) -> Result<User, ServerError> {
        todo!()
    }

    async fn delete_user(&self, _id: Uuid) -> Result<bool, ServerError> {
        todo!()
    }
}
