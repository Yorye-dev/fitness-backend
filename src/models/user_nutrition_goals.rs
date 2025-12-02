use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct UserNutritionsGoals {
    pub id: Uuid,
    pub user_id: Uuid,
    pub calorie_goal: f32,
    pub protein_goal: f32,
    pub fats_goal: f32,
    pub carbs_goal: f32,
    pub weight_goal: f32,
    pub tdee: f32,
    pub bmr: f32
}
