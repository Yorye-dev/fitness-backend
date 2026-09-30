use crate::domain::training::progress::WorkoutProgress;
use crate::{
    application::errors::ApplicationError,
    domain::{
        errors::DomainError,
        training::{
            session::{SessionUpdate, WorkoutSession},
            session_repository::{SessionWriteError, WorkoutSessionRepository},
        },
    },
};
use chrono::NaiveDate;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct WorkoutSessions(Arc<dyn WorkoutSessionRepository>);
impl WorkoutSessions {
    pub async fn progress(
        &self,
        user: Uuid,
        from: NaiveDate,
        to: NaiveDate,
        exercise: Option<Uuid>,
    ) -> Result<WorkoutProgress, ApplicationError> {
        if from > to || (to - from).num_days() > 365 || exercise.is_some_and(|id| id.is_nil()) {
            return Err(DomainError::Validation(
                "provide an ordered range of up to 366 days and a valid exercise id".into(),
            )
            .into());
        }
        Ok(self.0.progress(user, from, to, exercise).await?)
    }
    pub fn new(repo: Arc<dyn WorkoutSessionRepository>) -> Self {
        Self(repo)
    }
    pub async fn daily(
        &self,
        user: Uuid,
        date: NaiveDate,
    ) -> Result<Option<WorkoutSession>, ApplicationError> {
        Ok(self.0.daily(user, date).await?)
    }
    pub async fn start(
        &self,
        user: Uuid,
        date: NaiveDate,
        routine: Uuid,
    ) -> Result<WorkoutSession, ApplicationError> {
        if routine.is_nil() {
            return Err(DomainError::Validation("routine id must not be nil".into()).into());
        }
        Ok(self.0.start(user, date, routine).await?)
    }
    pub async fn save(
        &self,
        user: Uuid,
        id: Uuid,
        input: SessionUpdate,
    ) -> Result<WorkoutSession, ApplicationError> {
        input.validate()?;
        self.0.save(user, id, &input).await.map_err(|e| match e {
            SessionWriteError::Domain(e) => e.into(),
            SessionWriteError::Repository(e) => e.into(),
        })
    }
}
