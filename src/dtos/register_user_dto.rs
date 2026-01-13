use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct RegisterUserDto {
    pub username: String,
    pub plain_password: String,
    pub sex: String,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: String,
    pub goal: String,
}
