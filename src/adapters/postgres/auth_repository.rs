use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::adapters::auth::models::{RefreshToken, UserCredential, UserIdentity};
use crate::domain::error::ServerError;
use crate::ports::auth_repository::AuthRepository;

pub struct PostgresAuthRepository {
    pool: Pool<Postgres>,
}

impl PostgresAuthRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for PostgresAuthRepository {
    async fn find_identity_by_id(&self, _id: Uuid) -> Result<Option<UserIdentity>, ServerError> {
        todo!()
    }

    async fn find_identity_by_provider(
        &self,
        _provider_type: &str,
        _provider_id: &str,
    ) -> Result<Option<UserIdentity>, ServerError> {
        todo!()
    }

    async fn create_identity(&self, _identity: UserIdentity) -> Result<UserIdentity, ServerError> {
        todo!()
    }

    async fn delete_identity(&self, _id: Uuid) -> Result<bool, ServerError> {
        todo!()
    }

    async fn find_credential_by_identity(
        &self,
        _identity_id: Uuid,
    ) -> Result<Option<UserCredential>, ServerError> {
        todo!()
    }

    async fn create_credential(
        &self,
        _credential: UserCredential,
    ) -> Result<UserCredential, ServerError> {
        todo!()
    }

    async fn update_credential(
        &self,
        _credential: UserCredential,
    ) -> Result<UserCredential, ServerError> {
        todo!()
    }

    async fn increment_failed_attempts(&self, _identity_id: Uuid) -> Result<(), ServerError> {
        todo!()
    }

    async fn reset_failed_attempts(&self, _identity_id: Uuid) -> Result<(), ServerError> {
        todo!()
    }

    async fn find_refresh_token(
        &self,
        _token_hash: &str,
    ) -> Result<Option<RefreshToken>, ServerError> {
        todo!()
    }

    async fn find_refresh_tokens_by_user(
        &self,
        _user_id: Uuid,
    ) -> Result<Vec<RefreshToken>, ServerError> {
        todo!()
    }

    async fn create_refresh_token(
        &self,
        _token: RefreshToken,
    ) -> Result<RefreshToken, ServerError> {
        todo!()
    }

    async fn revoke_refresh_token(&self, _id: Uuid) -> Result<bool, ServerError> {
        todo!()
    }

    async fn revoke_all_user_tokens(&self, _user_id: Uuid) -> Result<u64, ServerError> {
        todo!()
    }
}
