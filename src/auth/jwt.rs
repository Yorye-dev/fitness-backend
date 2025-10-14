use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

use crate::auth::claims::Claims;

#[derive(Clone)]
pub struct Jwt {
    secret_key: String,
}

impl Jwt {

    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }

    pub fn generate_token(&self, user_id: &str, expiration_minutes: i64) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id, expiration_minutes);
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_bytes())
        )
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &Validation::default()
        )?;
        Ok(token_data.claims)
    }
}
