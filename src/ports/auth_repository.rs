use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::auth::models::{RefreshToken, UserCredential, UserIdentity};
use crate::domain::error::ServerError;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    // Identity
    async fn find_identity_by_id(&self, id: Uuid) -> Result<Option<UserIdentity>, ServerError>;
    async fn find_identity_by_provider(
        &self,
        provider_type: &str,
        provider_id: &str,
    ) -> Result<Option<UserIdentity>, ServerError>;
    async fn create_identity(&self, identity: UserIdentity) -> Result<UserIdentity, ServerError>;
    async fn delete_identity(&self, id: Uuid) -> Result<bool, ServerError>;

    // Credential
    async fn find_credential_by_identity(
        &self,
        identity_id: Uuid,
    ) -> Result<Option<UserCredential>, ServerError>;
    async fn create_credential(
        &self,
        credential: UserCredential,
    ) -> Result<UserCredential, ServerError>;
    async fn update_credential(
        &self,
        credential: UserCredential,
    ) -> Result<UserCredential, ServerError>;
    async fn increment_failed_attempts(&self, identity_id: Uuid) -> Result<(), ServerError>;
    async fn reset_failed_attempts(&self, identity_id: Uuid) -> Result<(), ServerError>;

    // Refresh Token
    async fn find_refresh_token(
        &self,
        token_hash: &str,
    ) -> Result<Option<RefreshToken>, ServerError>;
    async fn find_refresh_tokens_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<RefreshToken>, ServerError>;
    async fn create_refresh_token(&self, token: RefreshToken) -> Result<RefreshToken, ServerError>;
    async fn revoke_refresh_token(&self, id: Uuid) -> Result<bool, ServerError>;
    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<u64, ServerError>;
}
