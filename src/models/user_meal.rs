use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct UserMeal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub meal_id: Uuid,
    pub grams: f32,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_carbs: f32,
    pub total_fat: f32,
    pub created_at: NaiveDateTime,
}
