use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
//use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Meal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fat_per_100g: f32,
    //pub created_at: NaiveDateTime,
}

