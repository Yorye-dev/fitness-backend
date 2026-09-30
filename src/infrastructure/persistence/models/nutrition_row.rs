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
    pub per_unit: bool,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
}
#[derive(Debug, FromRow)]
pub struct ConsumptionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub meal_id: Uuid,
    pub quantity_grams: Option<f64>,
    pub portion_count: Option<f64>,
    pub portion_grams: Option<f64>,
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
    pub meal_calories: f32,
    pub meal_protein: f32,
    pub meal_carbs: f32,
    pub meal_fat: f32,
}
#[derive(Debug, FromRow)]
pub struct StatsRow {
    pub total_days: i64,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_carbs: f32,
    pub total_fat: f32,
}
