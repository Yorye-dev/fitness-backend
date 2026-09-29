use crate::domain::nutrition::goals::NutritionGoals;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct NutritionGoalsResponse {
    pub id: Uuid,
    pub protein_goal: f32,
    pub fats_goal: f32,
    pub carbs_goal: f32,
    pub tdee: f32,
    pub bmr: f32,
}
impl From<NutritionGoals> for NutritionGoalsResponse {
    fn from(goals: NutritionGoals) -> Self {
        Self {
            id: goals.id,
            protein_goal: goals.protein_goal,
            fats_goal: goals.fats_goal,
            carbs_goal: goals.carbs_goal,
            tdee: goals.tdee,
            bmr: goals.bmr,
        }
    }
}
