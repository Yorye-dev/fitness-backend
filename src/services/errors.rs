use thiserror::Error;
use sqlx::Error as SqlxError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Usuario no encontrado")]
    UserNotFound,

    #[error("Credenciales inválidas")]
    InvalidCredentials,

    #[error("Token inválido o expirado")]
    InvalidToken,

    #[error("Error de base de datos: {0}")]
    Database(#[from] SqlxError),

    #[error("Error inesperado: {0}")]
    Unexpected(String),
}

