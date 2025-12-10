use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::entities::user_nutrition_goals::UserNutritionsGoals;

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

    pub async fn save_user_goals(
        &self,
        user_nutritions_goals: &UserNutritionsGoals
    ) -> Result<UserNutritionsGoals, sqlx::Error> {

        let query = format!(
            "INSERT INTO {} (id, user_id, protein_goal, carbs_goal, fats_goal, tdee, bmr)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, protein_goal, carbs_goal, fats_goal, tdee, bmr",
            USER_NUTRITION_GOALS_TABLE
        );

        let saved_user_goals = sqlx::query_as::<_, UserNutritionsGoals>(&query)
            .bind(&user_nutritions_goals.id)
            .bind(&user_nutritions_goals.user_id)
            .bind(&user_nutritions_goals.protein_goal)
            .bind(&user_nutritions_goals.carbs_goal)
            .bind(&user_nutritions_goals.fats_goal)
            .bind(&user_nutritions_goals.tdee)
            .bind(&user_nutritions_goals.bmr)
            .fetch_one(&self.pool)
            .await?;

        Ok(saved_user_goals)
    }
}
