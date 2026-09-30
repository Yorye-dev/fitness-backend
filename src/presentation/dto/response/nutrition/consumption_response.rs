use crate::domain::nutrition::consumption::DailyConsumption;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ConsumptionResponse {
    pub id: Uuid,
    pub meal_id: Uuid,
    pub date: String,
    pub quantity_grams: Option<f64>,
    pub portion_count: Option<f64>,
    pub portion_grams: Option<f64>,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
}

impl From<DailyConsumption> for ConsumptionResponse {
    fn from(entry: DailyConsumption) -> Self {
        Self {
            id: entry.id,
            meal_id: entry.meal_id,
            date: entry.date.format("%Y-%m-%d").to_string(),
            quantity_grams: entry.quantity_grams,
            portion_count: entry.portion_count,
            portion_grams: entry.portion_grams,
            calories: entry.calories_consumed,
            protein: entry.protein_consumed,
            carbs: entry.carbs_consumed,
            fat: entry.fat_consumed,
        }
    }
}
