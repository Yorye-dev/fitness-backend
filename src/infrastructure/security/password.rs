use crate::application::security::password_service::{PasswordError, PasswordService};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use async_trait::async_trait;

pub struct Argon2PasswordService;

#[async_trait]
impl PasswordService for Argon2PasswordService {
    async fn hash(&self, password: &str) -> Result<String, PasswordError> {
        let password = password.to_owned();
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map(|hash| hash.to_string())
                .map_err(|_| PasswordError)
        })
        .await
        .map_err(|_| PasswordError)?
    }
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, PasswordError> {
        let password = password.to_owned();
        let hash = hash.to_owned();
        tokio::task::spawn_blocking(move || {
            let hash = PasswordHash::new(&hash).map_err(|_| PasswordError)?;
            match Argon2::default().verify_password(password.as_bytes(), &hash) {
                Ok(()) => Ok(true),
                Err(argon2::password_hash::Error::Password) => Ok(false),
                Err(_) => Err(PasswordError),
            }
        })
        .await
        .map_err(|_| PasswordError)?
    }
}
