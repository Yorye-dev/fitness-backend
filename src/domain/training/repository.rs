use super::routine::{WeeklyDay, WorkoutRoutine};
use crate::domain::errors::RepositoryError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TrainingRepository: Send + Sync {
    async fn list_routines(&self, user_id: Uuid) -> Result<Vec<WorkoutRoutine>, RepositoryError>;
    async fn save_routine(
        &self,
        user_id: Uuid,
        routine: &WorkoutRoutine,
    ) -> Result<WorkoutRoutine, RepositoryError>;
    async fn archive_routine(&self, user_id: Uuid, id: Uuid) -> Result<bool, RepositoryError>;
    async fn get_week(&self, user_id: Uuid) -> Result<Vec<WeeklyDay>, RepositoryError>;
    async fn save_week(
        &self,
        user_id: Uuid,
        days: &[WeeklyDay],
    ) -> Result<Vec<WeeklyDay>, RepositoryError>;
    async fn get_daily_routine(
        &self,
        user_id: Uuid,
        weekday: i16,
    ) -> Result<Option<WorkoutRoutine>, RepositoryError>;
}
