use sqlx::{Pool, Postgres};
use crate::models::user::User;
use uuid::Uuid;

pub struct UserRepository<'a> {
    pool: &'a Pool<Postgres>,
}

impl<'a> UserRepository<'a> {

    pub async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let rec sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, username, email, password_hash, age, sex, height_cm, weight_kg, activity_level)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(&user.username)
        .bind(&user.email)
        .fetch_one(self.pool)
        .await
    }
}
