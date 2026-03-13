use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::nutrition::goals::NutritionGoals;

#[async_trait]
pub trait NutritionRepository: Send + Sync {
    async fn get_user_goals(&self, user_id: &Uuid) -> Result<Option<NutritionGoals>, sqlx::Error>;
    async fn save_user_goals(&self, goals: &NutritionGoals) -> Result<NutritionGoals, sqlx::Error>;
}
