use crate::{
    application::errors::ApplicationError,
    domain::training::{
        repository::TrainingRepository,
        routine::{WeeklyDay, WorkoutRoutine, validate_week},
    },
};
use chrono::{Datelike, NaiveDate};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetWeeklyScheduleUseCase(Arc<dyn TrainingRepository>);
impl GetWeeklyScheduleUseCase {
    pub fn new(repository: Arc<dyn TrainingRepository>) -> Self {
        Self(repository)
    }
    pub async fn execute(&self, user_id: Uuid) -> Result<Vec<WeeklyDay>, ApplicationError> {
        Ok(self.0.get_week(user_id).await?)
    }
}
#[derive(Clone)]
pub struct SaveWeeklyScheduleUseCase(Arc<dyn TrainingRepository>);
impl SaveWeeklyScheduleUseCase {
    pub fn new(repository: Arc<dyn TrainingRepository>) -> Self {
        Self(repository)
    }
    pub async fn execute(
        &self,
        user_id: Uuid,
        days: Vec<WeeklyDay>,
    ) -> Result<Vec<WeeklyDay>, ApplicationError> {
        validate_week(&days)?;
        Ok(self.0.save_week(user_id, &days).await?)
    }
}
#[derive(Clone)]
pub struct GetDailyWorkoutUseCase(Arc<dyn TrainingRepository>);
impl GetDailyWorkoutUseCase {
    pub fn new(repository: Arc<dyn TrainingRepository>) -> Self {
        Self(repository)
    }
    pub async fn execute(
        &self,
        user_id: Uuid,
        date: NaiveDate,
    ) -> Result<Option<WorkoutRoutine>, ApplicationError> {
        Ok(self
            .0
            .get_daily_routine(user_id, date.weekday().number_from_monday() as i16)
            .await?)
    }
}
