use crate::application::errors::ApplicationError;
use crate::domain::{errors::DomainError, nutrition::repository::ConsumptionRepository};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeleteConsumptionUseCase {
    consumptions: Arc<dyn ConsumptionRepository>,
}

impl DeleteConsumptionUseCase {
    pub fn new(consumptions: Arc<dyn ConsumptionRepository>) -> Self {
        Self { consumptions }
    }

    pub async fn execute(&self, user_id: Uuid, id: Uuid) -> Result<(), ApplicationError> {
        if !self.consumptions.delete_consumption(&id, &user_id).await? {
            return Err(DomainError::ConsumptionNotFound.into());
        }
        Ok(())
    }
}
