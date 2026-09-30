use super::progress::WorkoutProgress;
use super::session::{SessionUpdate, WorkoutSession};
use crate::domain::errors::{DomainError, RepositoryError};
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum SessionWriteError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[async_trait]
pub trait WorkoutSessionRepository: Send + Sync {
    async fn progress(
        &self,
        user: Uuid,
        from: NaiveDate,
        to: NaiveDate,
        exercise: Option<Uuid>,
    ) -> Result<WorkoutProgress, RepositoryError>;
    async fn daily(
        &self,
        user: Uuid,
        date: NaiveDate,
    ) -> Result<Option<WorkoutSession>, RepositoryError>;
    async fn start(
        &self,
        user: Uuid,
        date: NaiveDate,
        routine: Uuid,
    ) -> Result<WorkoutSession, RepositoryError>;
    async fn save(
        &self,
        user: Uuid,
        id: Uuid,
        update: &SessionUpdate,
    ) -> Result<WorkoutSession, SessionWriteError>;
}
