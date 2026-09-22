use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUserNutritionGoalsDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub protein_goal: f32,
    pub fats_goal: f32,
    pub carbs_goal: f32,
    pub tdee: f32,
    pub bmr: f32,
}

impl From<crate::domain::nutrition::goals::NutritionGoals> for ResponseUserNutritionGoalsDto {
    fn from(goals: crate::domain::nutrition::goals::NutritionGoals) -> Self {
        ResponseUserNutritionGoalsDto {
            id: goals.id,
            user_id: goals.user_id,
            protein_goal: goals.protein_goal,
            fats_goal: goals.fats_goal,
            carbs_goal: goals.carbs_goal,
            tdee: goals.tdee,
            bmr: goals.bmr,
        }
    }
}
