use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserNutritionsGoalsDto {
    pub user_id: Uuid,
    pub protein_goal: f32,
    pub fats_goal: f32,
    pub carbs_goal: f32,
    pub tdee: f32,
    pub bmr: f32
}
