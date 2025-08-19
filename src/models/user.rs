use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::activity_level;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub sex: String,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: activity_level::ActivityLevel,
    
}
