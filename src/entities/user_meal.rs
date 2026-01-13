use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::super::enums::unit_type::UnitType;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct UserMeal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub meal_id: Uuid,
    pub unit: UnitType,
    pub total_calories: f32
}
