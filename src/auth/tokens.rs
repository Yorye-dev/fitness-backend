use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::auth::claims::Claims;

const SECRET: &[u8] = b"secret_key"; // TODO: Varibles de entorno
                                         //
pub fn generate_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        subject: user_id.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
