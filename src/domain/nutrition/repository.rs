use async_trait::async_trait;
use uuid::Uuid;
use chrono::NaiveDate;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::nutrition::meal::Meal;
use crate::domain::nutrition::consumption::DailyConsumption;

#[async_trait]
pub trait NutritionRepository: Send + Sync {
    async fn get_user_goals(&self, user_id: &Uuid) -> Result<Option<NutritionGoals>, sqlx::Error>;
    async fn save_user_goals(&self, goals: &NutritionGoals) -> Result<NutritionGoals, sqlx::Error>;
    async fn update_user_goals(&self, goals: &NutritionGoals) -> Result<NutritionGoals, sqlx::Error>;
}

#[async_trait]
pub trait MealRepository: Send + Sync {
    async fn save_meal(&self, meal: &Meal) -> Result<Meal, sqlx::Error>;
    async fn get_meal_by_id(&self, meal_id: &Uuid, user_id: &Uuid) -> Result<Option<Meal>, sqlx::Error>;
    async fn get_all_meals(&self, user_id: &Uuid) -> Result<Vec<Meal>, sqlx::Error>;
    async fn get_meals_paginated(&self, user_id: &Uuid, page: u32, per_page: u32) -> Result<(Vec<Meal>, i64), sqlx::Error>;
    async fn delete_meal(&self, meal_id: &Uuid, user_id: &Uuid) -> Result<bool, sqlx::Error>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConsumptionWithMeal {
    pub consumption: DailyConsumption,
    pub meal_name: String,
    pub meal_calories: f32,
    pub meal_protein: f32,
    pub meal_carbs: f32,
    pub meal_fat: f32,
}

#[async_trait]
pub trait ConsumptionRepository: Send + Sync {
    async fn log_consumption(&self, consumption: &DailyConsumption) -> Result<DailyConsumption, sqlx::Error>;
    async fn get_daily_consumption(&self, user_id: &Uuid, date: &NaiveDate) -> Result<Vec<DailyConsumption>, sqlx::Error>;
    async fn get_consumptions_paginated(&self, user_id: &Uuid, page: u32, per_page: u32, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>) -> Result<(Vec<ConsumptionWithMeal>, i64), sqlx::Error>;
    async fn get_consumptions_by_date_range(&self, user_id: &Uuid, start_date: &NaiveDate, end_date: &NaiveDate) -> Result<Vec<ConsumptionWithMeal>, sqlx::Error>;
    async fn get_date_range_stats(&self, user_id: &Uuid, start_date: &NaiveDate, end_date: &NaiveDate) -> Result<StatsSummary, sqlx::Error>;
    async fn delete_consumption(&self, consumption_id: &Uuid) -> Result<bool, sqlx::Error>;
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StatsSummary {
    pub total_days: i32,
    pub avg_calories: f32,
    pub avg_protein: f32,
    pub avg_carbs: f32,
    pub avg_fat: f32,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_carbs: f32,
    pub total_fat: f32,
}
