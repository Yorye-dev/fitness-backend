use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::user::user::{User, SignInUser};
use crate::domain::user::repository::UserRepository as UserRepositoryTrait;

#[derive(Clone)]
pub struct SqlxUserRepository {
    pool: Pool<Postgres>,
}

impl SqlxUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for SqlxUserRepository {
    async fn save_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let saved_user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, username, password_hash, age, sex, height, weight, activity_level, goal)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING id, username, password_hash, age, sex, height, weight, activity_level, goal"
        )
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.password_hash)
            .bind(user.age)
            .bind(&user.sex)
            .bind(user.height)
            .bind(user.weight)
            .bind(&user.activity_level)
            .bind(&user.goal)
            .fetch_one(&self.pool)
            .await?;

        Ok(saved_user)
    }

    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, age, sex, height, weight, activity_level, goal
             FROM users WHERE id = $1"
        )
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_user_by_username(&self, username: &String) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, age, sex, height, weight, activity_level, goal
             FROM users WHERE username = $1"
        )
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_sign_in_user_by_username(&self, username: &String) -> Result<Option<SignInUser>, sqlx::Error> {
        let user = sqlx::query_as::<_, SignInUser>(
            "SELECT id, username, password_hash FROM users WHERE username = $1"
        )
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    async fn exists(&self, user_id: &Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT EXISTS (SELECT 1 FROM users WHERE id = $1) as exists",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }
}
