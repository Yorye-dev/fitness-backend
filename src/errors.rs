use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("database error")]
    DatabaseError,

    #[error("user not found")]
    UserNotFound,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("password hashing error")]
    HashingError,
}
