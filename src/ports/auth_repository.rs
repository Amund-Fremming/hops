use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::auth::models::{RefreshToken, UserCredential, UserIdentity};

#[derive(Debug)]
pub enum AuthRepoError {
    NotFound,
    Conflict,
    DatabaseError(String),
}

#[async_trait]
pub trait AuthRepository: Send + Sync {
    // Identity
    async fn find_identity_by_id(&self, id: Uuid) -> Result<Option<UserIdentity>, AuthRepoError>;
    async fn find_identity_by_provider(
        &self,
        provider_type: &str,
        provider_id: &str,
    ) -> Result<Option<UserIdentity>, AuthRepoError>;
    async fn create_identity(&self, identity: UserIdentity) -> Result<UserIdentity, AuthRepoError>;
    async fn delete_identity(&self, id: Uuid) -> Result<bool, AuthRepoError>;

    // Credential
    async fn find_credential_by_identity(
        &self,
        identity_id: Uuid,
    ) -> Result<Option<UserCredential>, AuthRepoError>;
    async fn create_credential(
        &self,
        credential: UserCredential,
    ) -> Result<UserCredential, AuthRepoError>;
    async fn update_credential(
        &self,
        credential: UserCredential,
    ) -> Result<UserCredential, AuthRepoError>;
    async fn increment_failed_attempts(&self, identity_id: Uuid) -> Result<(), AuthRepoError>;
    async fn reset_failed_attempts(&self, identity_id: Uuid) -> Result<(), AuthRepoError>;

    // Refresh Token
    async fn find_refresh_token(
        &self,
        token_hash: &str,
    ) -> Result<Option<RefreshToken>, AuthRepoError>;
    async fn find_refresh_tokens_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<RefreshToken>, AuthRepoError>;
    async fn create_refresh_token(
        &self,
        token: RefreshToken,
    ) -> Result<RefreshToken, AuthRepoError>;
    async fn revoke_refresh_token(&self, id: Uuid) -> Result<bool, AuthRepoError>;
    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<u64, AuthRepoError>;
}
