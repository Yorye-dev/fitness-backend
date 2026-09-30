use crate::{
    application::errors::ApplicationError,
    domain::{
        errors::RepositoryError,
        training::{repository::TrainingRepository, routine::WorkoutRoutine},
    },
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct ListRoutinesUseCase(Arc<dyn TrainingRepository>);
impl ListRoutinesUseCase {
    pub fn new(repository: Arc<dyn TrainingRepository>) -> Self {
        Self(repository)
    }
    pub async fn execute(&self, user_id: Uuid) -> Result<Vec<WorkoutRoutine>, ApplicationError> {
        Ok(self.0.list_routines(user_id).await?)
    }
}
#[derive(Clone)]
pub struct SaveRoutineUseCase(Arc<dyn TrainingRepository>);
impl SaveRoutineUseCase {
    pub fn new(repository: Arc<dyn TrainingRepository>) -> Self {
        Self(repository)
    }
    pub async fn execute(
        &self,
        user_id: Uuid,
        routine: WorkoutRoutine,
    ) -> Result<WorkoutRoutine, ApplicationError> {
        Ok(self.0.save_routine(user_id, &routine.validate()?).await?)
    }
}
#[derive(Clone)]
pub struct ArchiveRoutineUseCase(Arc<dyn TrainingRepository>);
impl ArchiveRoutineUseCase {
    pub fn new(repository: Arc<dyn TrainingRepository>) -> Self {
        Self(repository)
    }
    pub async fn execute(&self, user_id: Uuid, id: Uuid) -> Result<(), ApplicationError> {
        if !self.0.archive_routine(user_id, id).await? {
            return Err(RepositoryError::NotFound.into());
        }
        Ok(())
    }
}
