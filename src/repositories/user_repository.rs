use crate::models::user::{User, NewUser};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use anyhow::Result;
use argon2::{self, Config};

pub struct UserRepository;

impl UserRepository {
    pub async fn create_user(pool: &PgPool, new_user: NewUser) -> Result<User> {
        let user_id = Uuid::new_v4();

        // hasheamos password antes de insertar
        let salt = b"randomsalt"; // 👈 mejor generar un salt único en prod
        let password_hash = argon2::hash_encoded(new_user.password.as_bytes(), salt, &Config::default())?;

        let record = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                id, username, email, password_hash, age, sex,
                height_cm, weight_kg, activity_level, created_at
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(&new_user.username)
        .bind(&new_user.email)
        .bind(password_hash)
        .bind(new_user.age)
        .bind(&new_user.sex)
        .bind(new_user.height_cm)
        .bind(new_user.weight_kg)
        .bind(&new_user.activity_level)
        .bind(Utc::now().naive_utc())
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}

