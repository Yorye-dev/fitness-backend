use super::DailyWater;
use crate::domain::errors::RepositoryError;
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

#[async_trait]
pub trait HydrationRepository: Send + Sync {
    async fn daily(&self, user: Uuid, date: NaiveDate) -> Result<DailyWater, RepositoryError>;
    async fn add(
        &self,
        user: Uuid,
        date: NaiveDate,
        id: Uuid,
        amount: i32,
    ) -> Result<DailyWater, RepositoryError>;
    async fn remove(&self, user: Uuid, id: Uuid) -> Result<(), RepositoryError>;
    async fn set_goal(
        &self,
        user: Uuid,
        date: NaiveDate,
        goal: i32,
    ) -> Result<DailyWater, RepositoryError>;
}
