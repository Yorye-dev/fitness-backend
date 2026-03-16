use async_trait::async_trait;
use uuid::Uuid;
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
    async fn get_meal_by_id(&self, meal_id: &Uuid) -> Result<Option<Meal>, sqlx::Error>;
    async fn get_all_meals(&self) -> Result<Vec<Meal>, sqlx::Error>;
    async fn delete_meal(&self, meal_id: &Uuid) -> Result<bool, sqlx::Error>;
}

#[async_trait]
pub trait ConsumptionRepository: Send + Sync {
    async fn log_consumption(&self, consumption: &DailyConsumption) -> Result<DailyConsumption, sqlx::Error>;
    async fn get_daily_consumption(&self, user_id: &Uuid, date: &chrono::NaiveDate) -> Result<Vec<DailyConsumption>, sqlx::Error>;
    async fn delete_consumption(&self, consumption_id: &Uuid) -> Result<bool, sqlx::Error>;
}
