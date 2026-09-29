use crate::domain::errors::RepositoryError;

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        match &error {
            sqlx::Error::RowNotFound => Self::NotFound,
            sqlx::Error::Database(db)
                if matches!(db.code().as_deref(), Some("23505" | "23503")) =>
            {
                Self::Conflict
            }
            sqlx::Error::PoolTimedOut
            | sqlx::Error::PoolClosed
            | sqlx::Error::Io(_)
            | sqlx::Error::Tls(_) => {
                eprintln!("Database unavailable: {error}");
                Self::Unavailable
            }
            _ => {
                eprintln!("Database operation failed: {error}");
                Self::Unexpected
            }
        }
    }
}
