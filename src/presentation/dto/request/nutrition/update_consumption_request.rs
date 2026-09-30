use super::daily_nutrition_query::DailyNutritionQuery;
use crate::{
    application::nutrition::{
        consumption_input::ConsumptionQuantityInput, update_consumption::UpdateConsumptionInput,
    },
    presentation::errors::api_error::ApiError,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateConsumptionRequest {
    pub date: String,
    pub quantity_grams: Option<f64>,
    pub portion_count: Option<f64>,
    pub portion_grams: Option<f64>,
}

impl TryFrom<UpdateConsumptionRequest> for UpdateConsumptionInput {
    type Error = ApiError;

    fn try_from(request: UpdateConsumptionRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            date: DailyNutritionQuery { date: request.date }.parsed_date()?,
            quantity: ConsumptionQuantityInput {
                quantity_grams: request.quantity_grams,
                portion_count: request.portion_count,
                portion_grams: request.portion_grams,
            },
        })
    }
}
