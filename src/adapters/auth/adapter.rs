use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::warn;
use uuid::Uuid;

use crate::adapters::auth::models::{Jwk, Jwks};
use crate::domain::error::ServerError;
use crate::ports::auth_port::{AuthPort, Claims, TokenResponse};
use crate::ports::auth_repository::AuthRepository;

pub struct JwtAdapter {
    repo: Arc<dyn AuthRepository>,
    jwks: Jwks,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    audience: String,
    issuer: String,
}

impl JwtAdapter {
    pub fn new(
        repo: Arc<dyn AuthRepository>,
        private_key_pem: &str,
        public_key_pem: &str,
        audience: &str,
        issuer: &str,
    ) -> Result<Self, ServerError> {
        let encoding_key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
            .map_err(|e| ServerError::Auth(format!("Invalid private key: {}", e)))?;

        let decoding_key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())
            .map_err(|e| ServerError::Auth(format!("Invalid public key: {}", e)))?;

        let jwk1 = Jwk::new("key-1", public_key_pem)?;
        let jwk2 = Jwk::new("key-2", public_key_pem)?;
        let jwks = Jwks { keys: [jwk1, jwk2] };

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[audience]);
        validation.set_issuer(&[issuer]);

        Ok(Self {
            repo,
            jwks,
            encoding_key,
            decoding_key,
            validation,
            audience: audience.to_string(),
            issuer: issuer.to_string(),
        })
    }
}

#[async_trait]
impl AuthPort for JwtAdapter {
    fn get_jwks(&self) -> &Jwks {
        &self.jwks
    }

    fn validate_token(&self, token: &str) -> Result<Claims, StatusCode> {
        let token_data =
            decode::<Claims>(token, &self.decoding_key, &self.validation).map_err(|e| {
                warn!("Token validation failed: {}", e);
                StatusCode::UNAUTHORIZED
            })?;
        Ok(token_data.claims)
    }

    fn generate_tokens(&self, user_id: Uuid) -> Result<TokenResponse, ServerError> {
        let access_token_lifetime = 16 * 60; // 15 mins

        let claims = Claims {
            sub: user_id.to_string(),
            iss: self.issuer.clone(),
            aud: vec![self.audience.clone()],
            exp: (Utc::now().timestamp() + access_token_lifetime) as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let header = Header::new(Algorithm::RS256);
        let access_token = encode(&header, &claims, &self.encoding_key)
            .map_err(|e| ServerError::Auth(format!("Failed to encode AT: {:?}", e)))?;

        let refresh_token = {
            let bytes: [u8; 32] = rand::random();
            URL_SAFE_NO_PAD.encode(bytes)
        };

        Ok(TokenResponse {
            access_token,
            refresh_token,
            expires_in: access_token_lifetime,
        })
    }

    async fn authenticate(
        &self,
        phone: &str,
        password: &str,
    ) -> Result<TokenResponse, ServerError> {
        let user_id = Uuid::new_v4(); // TODO
        self.generate_tokens(user_id)
    }

    async fn get_identities(&self, user_id: Uuid) -> Result<Vec<String>, ServerError> {
        todo!()
    }

    async fn set_password(
        &self,
        user_id: Uuid,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), ServerError> {
        todo!()
    }

    async fn rotate_tokens(&self, refresh_token: &str) -> Result<TokenResponse, ServerError> {
        todo!()
    }

    async fn increment_failed_attempts(&self, user_id: Uuid) -> Result<(), ServerError> {
        todo!()
    }

    async fn reset_failed_attempts(&self, user_id: Uuid) -> Result<(), ServerError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::auth::models::{RefreshToken, UserCredential, UserIdentity};
    use crate::ports::auth_repository::AuthRepository;
    use std::sync::Arc;

    struct MockAuthRepository;

    #[async_trait]
    impl AuthRepository for MockAuthRepository {
        async fn find_identity_by_id(
            &self,
            _id: Uuid,
        ) -> Result<Option<UserIdentity>, ServerError> {
            Ok(None)
        }
        async fn find_identity_by_provider(
            &self,
            _provider_type: &str,
            _provider_id: &str,
        ) -> Result<Option<UserIdentity>, ServerError> {
            Ok(None)
        }
        async fn create_identity(
            &self,
            identity: UserIdentity,
        ) -> Result<UserIdentity, ServerError> {
            Ok(identity)
        }
        async fn delete_identity(&self, _id: Uuid) -> Result<bool, ServerError> {
            Ok(true)
        }
        async fn find_credential_by_identity(
            &self,
            _identity_id: Uuid,
        ) -> Result<Option<UserCredential>, ServerError> {
            Ok(None)
        }
        async fn create_credential(
            &self,
            credential: UserCredential,
        ) -> Result<UserCredential, ServerError> {
            Ok(credential)
        }
        async fn update_credential(
            &self,
            credential: UserCredential,
        ) -> Result<UserCredential, ServerError> {
            Ok(credential)
        }
        async fn increment_failed_attempts(&self, _identity_id: Uuid) -> Result<(), ServerError> {
            Ok(())
        }
        async fn reset_failed_attempts(&self, _identity_id: Uuid) -> Result<(), ServerError> {
            Ok(())
        }
        async fn find_refresh_token(
            &self,
            _token_hash: &str,
        ) -> Result<Option<RefreshToken>, ServerError> {
            Ok(None)
        }
        async fn find_refresh_tokens_by_user(
            &self,
            _user_id: Uuid,
        ) -> Result<Vec<RefreshToken>, ServerError> {
            Ok(vec![])
        }
        async fn create_refresh_token(
            &self,
            token: RefreshToken,
        ) -> Result<RefreshToken, ServerError> {
            Ok(token)
        }
        async fn revoke_refresh_token(&self, _id: Uuid) -> Result<bool, ServerError> {
            Ok(true)
        }
        async fn revoke_all_user_tokens(&self, _user_id: Uuid) -> Result<u64, ServerError> {
            Ok(0)
        }
    }

    fn create_test_adapter() -> JwtAdapter {
        use rsa::RsaPrivateKey;
        use rsa::pkcs1::{EncodeRsaPrivateKey, LineEnding};
        use rsa::pkcs8::EncodePublicKey;

        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("generate key");
        let public_key = private_key.to_public_key();

        let private_pem = private_key
            .to_pkcs1_pem(LineEnding::LF)
            .expect("encode private key");
        let public_pem = public_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .expect("encode public key");

        let repo: Arc<dyn AuthRepository> = Arc::new(MockAuthRepository);
        JwtAdapter::new(
            repo,
            private_pem.as_str(),
            &public_pem,
            "test-audience",
            "test-issuer",
        )
        .expect("create adapter")
    }

    #[test]
    fn generate_tokens_creates_valid_access_token() {
        let adapter = create_test_adapter();
        let user_id = Uuid::new_v4();

        let token_response = adapter.generate_tokens(user_id).expect("generate tokens");

        assert!(!token_response.access_token.is_empty());
        assert!(!token_response.refresh_token.is_empty());
        assert_eq!(token_response.expires_in, 16 * 60);
    }

    #[test]
    fn generated_token_can_be_decoded() {
        let adapter = create_test_adapter();
        let user_id = Uuid::new_v4();

        let token_response = adapter.generate_tokens(user_id).expect("generate tokens");
        let claims = adapter
            .validate_token(&token_response.access_token)
            .expect("validate token");

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.iss, "test-issuer");
        assert_eq!(claims.aud, vec!["test-audience"]);
    }

    #[test]
    fn refresh_token_is_random_base64() {
        let adapter = create_test_adapter();
        let user_id = Uuid::new_v4();

        let token1 = adapter.generate_tokens(user_id).expect("generate tokens");
        let token2 = adapter.generate_tokens(user_id).expect("generate tokens");

        // Refresh tokens should be different each time
        assert_ne!(token1.refresh_token, token2.refresh_token);

        // Should be valid base64
        URL_SAFE_NO_PAD
            .decode(&token1.refresh_token)
            .expect("decode refresh token");
    }
}
