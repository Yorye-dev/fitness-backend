use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::enums::activity_level;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: activity_level::ActivityLevel,
    pub created_at: DateTime<Utc>,
}
