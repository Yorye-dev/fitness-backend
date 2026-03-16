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
    pub consumed: MacroSummary,
    pub goals: MacroSummary,
    pub remaining: MacroSummary,
    pub progress_percentage: f32,
    pub macros: MacroProgress,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroSummary {
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroProgress {
    pub calories_percent: f32,
    pub protein_percent: f32,
    pub carbs_percent: f32,
    pub fat_percent: f32,
}
