use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {

    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("user not found")]
    UserNotFound,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("password hashing error")]
    HashingError,

    #[error("generate token error")]
    GenerateTokenError,

    #[error("unauthorized token")]
    Unauthorized    
}
