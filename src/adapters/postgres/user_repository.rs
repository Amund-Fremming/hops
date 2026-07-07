use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::user::User;
use crate::ports::user_repository::{UserRepoError, UserRepository};

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
    async fn get_user(&self, _id: Uuid) -> Result<Option<User>, UserRepoError> {
        Ok(None)
    }

    async fn get_all_users(&self) -> Result<Vec<User>, UserRepoError> {
        todo!()
    }

    async fn create_user(&self, _user: User) -> Result<User, UserRepoError> {
        todo!()
    }

    async fn update_user(&self, _user: User) -> Result<User, UserRepoError> {
        todo!()
    }

    async fn delete_user(&self, _id: Uuid) -> Result<bool, UserRepoError> {
        todo!()
    }
}
