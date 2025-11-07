use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::user_nutrition_goals::UserNutritionsGoals;

const USER_TABLE: &str = "users";

#[derive(Clone)]
pub struct GoalsRepository {
    pool: Pool<Postgres>,
}

impl GoalsRepository  {
    
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_user_goals(&self, user_id:Uuid) -> Result<Option<UserNutritionsGoals>, sqlx::Error>{

        Ok(())
    }
}
