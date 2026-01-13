use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct UserNutritionsGoals {
    pub id: Uuid,
    pub user_id: Uuid,
    pub protein_goal: f32,
    pub fats_goal: f32,
    pub carbs_goal: f32,
    pub tdee: f32,
    pub bmr: f32
}

impl UserNutritionsGoals {

    pub fn new(user_id: Uuid, protein_goal: f32, fats_goal: f32, carbs_goal: f32, tdee: f32, bmr: f32) -> Self {
        Self {
            id : Uuid::new_v4(),
            user_id,
            protein_goal,
            fats_goal,
            carbs_goal,
            tdee,
            bmr,
        }
    }

    pub fn from_db(
        id: Uuid,
        user_id: Uuid,
        protein_goal: f32,
        fats_goal: f32,
        carbs_goal: f32,
        tdee: f32,
        bmr: f32,
    ) -> Self {
        Self {
            id,
            user_id,
            protein_goal,
            fats_goal,
            carbs_goal,
            tdee,
            bmr,
        }
    }
}
