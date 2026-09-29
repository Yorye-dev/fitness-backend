use super::claims::{Claims, TokenType};
use crate::application::security::token_service::{TokenError, TokenIdentity, TokenService};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;

pub struct JwtTokenService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtTokenService {
    pub fn new(secret_key: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret_key.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret_key.as_bytes()),
        }
    }

    fn generate(
        &self,
        user_id: &Uuid,
        expiration_minutes: i64,
        token_type: TokenType,
    ) -> Result<String, TokenError> {
        let claims = Claims::new(&user_id.to_string(), expiration_minutes, token_type);
        encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(|_| TokenError::Generation)
    }

    fn validate(&self, token: &str, expected_type: TokenType) -> Result<TokenIdentity, TokenError> {
        let claims = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| TokenError::Invalid)?
        .claims;
        if claims.token_type != expected_type {
            return Err(TokenError::Invalid);
        }
        let user_id = Uuid::parse_str(&claims.subject).map_err(|_| TokenError::Invalid)?;
        Ok(TokenIdentity { user_id })
    }
}

impl TokenService for JwtTokenService {
    fn create_access_token(&self, user_id: &Uuid) -> Result<String, TokenError> {
        self.generate(user_id, 60, TokenType::Access)
    }

    fn create_refresh_token(&self, user_id: &Uuid) -> Result<String, TokenError> {
        self.generate(user_id, 10080, TokenType::Refresh)
    }

    fn validate_access_token(&self, token: &str) -> Result<TokenIdentity, TokenError> {
        self.validate(token, TokenType::Access)
    }

    fn validate_refresh_token(&self, token: &str) -> Result<TokenIdentity, TokenError> {
        self.validate(token, TokenType::Refresh)
    }
}
