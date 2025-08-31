use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub sex: String,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: String,   
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: String,
    pub username: String,
    pub sex: String,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: String,   
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        PublicUser {
            id: user.id,
            sex: user.sex,
            username: user.username,
            weight: user.weight,
            height: user.height,
            age: user.age,
            activity_level: user.activity_level,
        }
    }
}
