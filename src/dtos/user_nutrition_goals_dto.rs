use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct UserNutritionsGoalsResponseDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub protein_goal: f32,
    pub fats_goal: f32,
    pub carbs_goal: f32,
    pub tdee: f32,
    pub bmr: f32,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserNutritionsGoalsDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub protein_goal: f32,
    pub fats_goal: f32,
    pub carbs_goal: f32,
    pub tdee: f32,
    pub bmr: f32,
}
