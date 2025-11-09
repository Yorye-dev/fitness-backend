use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::user_nutrition_goals::UserNutritionsGoals;

const USER_TABLE: &str = "users_nutrition_goals";

#[derive(Clone)]
pub struct GoalsRepository {
    pool: Pool<Postgres>,
}

impl GoalsRepository  {
    
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /*
    pub async fn get_user_goals(&self, user_id: Uuid) -> sqlx::Result<Option<UserNutritionsGoals>> {
        sqlx::query_as!(
            UserNutritionsGoals,
            r#"
            SELECT user_id, calories_goal, protein_goal, carbs_goal, fats_goal
            FROM user_goals
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
    }
    */
}
