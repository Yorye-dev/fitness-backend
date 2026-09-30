use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("user not found")]
    UserNotFound,
    #[error("meal not found")]
    MealNotFound,
    #[error("consumption not found")]
    ConsumptionNotFound,
    #[error("nutrition goals not found")]
    NutritionGoalsNotFound,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("{0}")]
    Validation(String),
}

/// Storage failures independent of the database driver.
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("resource already exists or is still referenced")]
    Conflict,
    #[error("resource not found")]
    NotFound,
    #[error("storage unavailable")]
    Unavailable,
    #[error("unexpected storage error")]
    Unexpected,
}
