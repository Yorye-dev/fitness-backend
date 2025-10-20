use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
//use chrono::NaiveDateTime;

use super::super::enums::unit_type::UnitType;


#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Meal {
    pub id: Uuid,
    pub name: String,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fat_per_100g: f32,
    pub default_quantity: UnitType,
    pub default_unit: f32
    //pub created_at: NaiveDateTime,
}
