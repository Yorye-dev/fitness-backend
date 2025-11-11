use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::user_nutrition_goals::UserNutritionsGoals;

const USER_NUTRITION_GOALS_TABLE: &str = "users_nutrition_goals";

#[derive(Clone)]
pub struct GoalsRepository {
    pool: Pool<Postgres>,
}

impl GoalsRepository  {
    
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_user_goals (&self, user_id: &Uuid) -> Result<Option<UserNutritionsGoals>, sqlx::Error> {
        let query = format!("SELECT *
            FROM {}
            WHERE user_id = $1", USER_NUTRITION_GOALS_TABLE);
        
        let user = sqlx::query_as::<_, UserNutritionsGoals>(&query)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}
