use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

use crate::auth::claims::Claims;


pub fn generate_token(user_id: &String) -> Result<String, jsonwebtoken::errors::Error> {
    
    let secret_key = env::var("SECRET_KEY").expect("Error en .env falta SECRET_KEY");
    let claims = Claims::new(user_id.to_string(), 60);
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_bytes()))
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret_key = env::var("SECRET_KEY").expect("Error en .env falta SECRET_KEY");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
