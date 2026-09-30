use super::consumption_input::ConsumptionQuantityInput;
use crate::{
    application::errors::ApplicationError,
    domain::{
        errors::DomainError,
        nutrition::{consumption::DailyConsumption, repository::ConsumptionRepository},
    },
};
use chrono::NaiveDate;
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateConsumptionInput {
    pub date: NaiveDate,
    pub quantity: ConsumptionQuantityInput,
}

#[derive(Clone)]
pub struct UpdateConsumptionUseCase {
    consumptions: Arc<dyn ConsumptionRepository>,
}

impl UpdateConsumptionUseCase {
    pub fn new(consumptions: Arc<dyn ConsumptionRepository>) -> Self {
        Self { consumptions }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        id: Uuid,
        input: UpdateConsumptionInput,
    ) -> Result<DailyConsumption, ApplicationError> {
        let quantity = input.quantity.resolve()?;
        self.consumptions
            .update_consumption(&id, &user_id, &input.date, &quantity)
            .await?
            .ok_or_else(|| DomainError::ConsumptionNotFound.into())
    }
}
