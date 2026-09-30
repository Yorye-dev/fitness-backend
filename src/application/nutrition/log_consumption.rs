use super::consumption_input::ConsumptionQuantityInput;
use crate::application::errors::ApplicationError;
use crate::domain::{
    errors::{DomainError, RepositoryError},
    nutrition::{consumption::DailyConsumption, repository::ConsumptionRepository},
};
use chrono::NaiveDate;
use std::sync::Arc;
use uuid::Uuid;

pub struct LogConsumptionInput {
    pub id: Uuid,
    pub meal_id: Uuid,
    pub date: NaiveDate,
    pub quantity: ConsumptionQuantityInput,
}

#[derive(Clone)]
pub struct LogConsumptionUseCase {
    consumptions: Arc<dyn ConsumptionRepository>,
}

impl LogConsumptionUseCase {
    pub fn new(consumptions: Arc<dyn ConsumptionRepository>) -> Self {
        Self { consumptions }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        input: LogConsumptionInput,
    ) -> Result<DailyConsumption, ApplicationError> {
        if input.id.is_nil() || input.meal_id.is_nil() {
            return Err(DomainError::Validation("ids must not be nil".into()).into());
        }
        let quantity = input.quantity.resolve()?;
        // Persistence snapshots the owned food and calculates totals atomically.
        // The client never supplies user identity or nutrient totals.
        let consumption = DailyConsumption {
            id: input.id,
            user_id,
            date: input.date,
            meal_id: input.meal_id,
            quantity_grams: quantity.quantity_grams,
            portion_count: quantity.portion_count,
            portion_grams: quantity.portion_grams,
            calories_consumed: 0.0,
            protein_consumed: 0.0,
            carbs_consumed: 0.0,
            fat_consumed: 0.0,
        };
        self.consumptions
            .log_consumption(&consumption)
            .await
            .map_err(|error| match error {
                RepositoryError::NotFound => DomainError::MealNotFound.into(),
                error => error.into(),
            })
    }
}
