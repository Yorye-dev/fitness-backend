use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct DailyConsumption {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: chrono::NaiveDate,
    pub meal_id: Uuid,
    pub quantity_grams: f32,
    pub calories_consumed: f32,
    pub protein_consumed: f32,
    pub carbs_consumed: f32,
    pub fat_consumed: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailySummary {
    pub date: String,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_carbs: f32,
    pub total_fat: f32,
    pub goal_calories: f32,
    pub goal_protein: f32,
    pub goal_carbs: f32,
    pub goal_fat: f32,
    pub remaining_calories: f32,
    pub remaining_protein: f32,
    pub remaining_carbs: f32,
    pub remaining_fat: f32,
    pub meals_consumed: Vec<ConsumedMeal>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConsumedMeal {
    pub meal_id: Uuid,
    pub meal_name: String,
    pub quantity_grams: f32,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
}
