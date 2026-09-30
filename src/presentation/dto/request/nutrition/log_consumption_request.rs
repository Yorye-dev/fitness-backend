use super::daily_nutrition_query::DailyNutritionQuery;
use crate::{
    application::nutrition::{
        consumption_input::ConsumptionQuantityInput, log_consumption::LogConsumptionInput,
    },
    presentation::errors::api_error::ApiError,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogConsumptionRequest {
    pub id: Uuid,
    pub meal_id: Uuid,
    pub date: String,
    pub quantity_grams: Option<f64>,
    pub portion_count: Option<f64>,
    pub portion_grams: Option<f64>,
}

impl TryFrom<LogConsumptionRequest> for LogConsumptionInput {
    type Error = ApiError;

    fn try_from(request: LogConsumptionRequest) -> Result<Self, Self::Error> {
        let date = DailyNutritionQuery { date: request.date }.parsed_date()?;
        Ok(Self {
            id: request.id,
            meal_id: request.meal_id,
            date,
            quantity: ConsumptionQuantityInput {
                quantity_grams: request.quantity_grams,
                portion_count: request.portion_count,
                portion_grams: request.portion_grams,
            },
        })
    }
}
