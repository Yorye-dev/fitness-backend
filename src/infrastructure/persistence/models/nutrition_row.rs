use chrono::NaiveDate;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct NutritionGoalsRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub protein_goal: f32,
    pub fats_goal: f32,
    pub carbs_goal: f32,
    pub tdee: f32,
    pub bmr: f32,
}
#[derive(Debug, FromRow)]
pub struct MealRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fat_per_100g: f32,
}
#[derive(Debug, FromRow)]
pub struct ConsumptionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub meal_id: Uuid,
    pub quantity_grams: f32,
    pub calories_consumed: f32,
    pub protein_consumed: f32,
    pub carbs_consumed: f32,
    pub fat_consumed: f32,
}
#[derive(Debug, FromRow)]
pub struct ConsumptionWithMealRow {
    #[sqlx(flatten)]
    pub consumption: ConsumptionRow,
    pub meal_name: String,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fat_per_100g: f32,
}
#[derive(Debug, FromRow)]
pub struct StatsRow {
    pub total_days: i64,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_carbs: f32,
    pub total_fat: f32,
}
