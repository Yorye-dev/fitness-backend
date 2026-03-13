use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::nutrition::repository::NutritionRepository as NutritionRepositoryTrait;

const GOALS_TABLE: &str = "users_nutrition_goals";

pub struct SqlxNutritionRepository {
    pool: Pool<Postgres>,
}

impl SqlxNutritionRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NutritionRepositoryTrait for SqlxNutritionRepository {
    async fn get_user_goals(&self, user_id: &Uuid) -> Result<Option<NutritionGoals>, sqlx::Error> {
        let query = format!("SELECT *
            FROM {}
            WHERE user_id = $1", GOALS_TABLE);
        
        let goals = sqlx::query_as::<_, NutritionGoals>(&query)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(goals)
    }

    async fn save_user_goals(&self, goals: &NutritionGoals) -> Result<NutritionGoals, sqlx::Error> {
        let query = format!(
            "INSERT INTO {} (id, user_id, protein_goal, carbs_goal, fats_goal, tdee, bmr)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, protein_goal, carbs_goal, fats_goal, tdee, bmr",
            GOALS_TABLE
        );

        let saved_goals = sqlx::query_as::<_, NutritionGoals>(&query)
            .bind(&goals.id)
            .bind(&goals.user_id)
            .bind(&goals.protein_goal)
            .bind(&goals.carbs_goal)
            .bind(&goals.fats_goal)
            .bind(&goals.tdee)
            .bind(&goals.bmr)
            .fetch_one(&self.pool)
            .await?;

        Ok(saved_goals)
    }
}
