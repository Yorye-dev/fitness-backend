use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::user::User;

pub struct UserRepository<'a> {
    pool: &'a Pool<Postgres>
}

impl<'a> UserRepository<'a> {

    pub async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, password_hash, age, sex, height, weight, activity_level)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            id
            username,
            email,
            password_hash,
            age,
            sex,
            height_cm,
            weight_kg,
            activity_level,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(user_id)
    }
}
