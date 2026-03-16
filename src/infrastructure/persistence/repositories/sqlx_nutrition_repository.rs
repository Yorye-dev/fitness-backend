use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::nutrition::repository::NutritionRepository as NutritionRepositoryTrait;

#[derive(Clone)]
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
        let goals = sqlx::query_as::<_, NutritionGoals>(
            "SELECT id, user_id, protein_goal, carbs_goal, fats_goal, tdee, bmr
             FROM users_nutrition_goals WHERE user_id = $1"
        )
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(goals)
    }

    async fn save_user_goals(&self, goals: &NutritionGoals) -> Result<NutritionGoals, sqlx::Error> {
        let saved_goals = sqlx::query_as::<_, NutritionGoals>(
            "INSERT INTO users_nutrition_goals (id, user_id, protein_goal, carbs_goal, fats_goal, tdee, bmr)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING id, user_id, protein_goal, carbs_goal, fats_goal, tdee, bmr"
        )
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
