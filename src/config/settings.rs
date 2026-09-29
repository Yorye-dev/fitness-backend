use std::{env, fs, net::SocketAddr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing or empty configuration variable: {0}")]
    Missing(&'static str),
    #[error("cannot read secret file for {0}")]
    SecretFile(&'static str),
    #[error("BIND_ADDRESS must contain a valid IP address and port")]
    BindAddress,
}
pub struct Settings {
    pub database_url: String,
    pub secret_key: String,
    pub bind_address: SocketAddr,
    pub project_name: String,
    pub cors_allowed_origins: Vec<String>,
}
impl Settings {
    pub fn from_env() -> Result<Self, ConfigError> {
        let bind = env::var("BIND_ADDRESS")
            .or_else(|_| env::var("APP_URL"))
            .map_err(|_| ConfigError::Missing("BIND_ADDRESS"))?;
        Ok(Self {
            database_url: env_or_file("DATABASE_URL", "DATABASE_URL_FILE")?,
            secret_key: env_or_file("SECRET_KEY", "SECRET_KEY_FILE")?,
            bind_address: bind.parse().map_err(|_| ConfigError::BindAddress)?,
            project_name: env::var("PROJECT_NAME")
                .map_err(|_| ConfigError::Missing("PROJECT_NAME"))?,
            cors_allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .unwrap_or_default()
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect(),
        })
    }
}
fn env_or_file(variable: &'static str, file_variable: &'static str) -> Result<String, ConfigError> {
    let value = match env::var(file_variable) {
        Ok(path) => fs::read_to_string(path).map_err(|_| ConfigError::SecretFile(variable))?,
        Err(_) => env::var(variable).map_err(|_| ConfigError::Missing(variable))?,
    };
    let value = value.trim().to_owned();
    if value.is_empty() {
        return Err(ConfigError::Missing(variable));
    }
    Ok(value)
}
