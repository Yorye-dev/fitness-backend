use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::auth::claims::Claims;

#[derive(Clone)]
pub struct Jwt {
    secret_key: String,
}

impl Jwt {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }

    pub fn generate_token(
        &self,
        user_id: &Uuid,
        expiration_minutes: i64,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(&user_id.to_string(), expiration_minutes);
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_bytes()),
        )
    }

    pub fn generate_access_token(
        &self,
        user_id: &Uuid,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        self.generate_token(user_id, 60)
    }

    pub fn generate_refresh_token(
        &self,
        user_id: &Uuid,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        self.generate_token(user_id, 10080)
    }

    pub fn decode_token(&self, token: &String) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}
