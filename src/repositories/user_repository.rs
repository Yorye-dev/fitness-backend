use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::user::User;

pub struct UserRepository<'a> {
    pool: &'a Pool<Postgres>
}

impl<'a> UserRepository<'a> {

    pub async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let row = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, email, password_hash, age, sex, height, weight, activity_level)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, username, email, password_hash, age, sex, height_cm, weight_kg, activity_level, created_at
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.age,
            user.sex,
            user.height,
            user.weight,
            user.activity_level,
            user.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }
}
