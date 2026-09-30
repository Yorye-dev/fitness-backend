use crate::{
    application::errors::ApplicationError,
    domain::{
        errors::DomainError,
        hydration::{DailyWater, repository::HydrationRepository},
    },
};
use chrono::NaiveDate;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct Hydration(Arc<dyn HydrationRepository>);
impl Hydration {
    pub fn new(repo: Arc<dyn HydrationRepository>) -> Self {
        Self(repo)
    }
    pub async fn daily(&self, user: Uuid, date: NaiveDate) -> Result<DailyWater, ApplicationError> {
        Ok(self.0.daily(user, date).await?)
    }
    pub async fn add(
        &self,
        user: Uuid,
        date: NaiveDate,
        id: Uuid,
        amount: i32,
    ) -> Result<DailyWater, ApplicationError> {
        if id.is_nil() || !(1..=5000).contains(&amount) {
            return Err(DomainError::Validation("water amount must be 1–5000 ml".into()).into());
        }
        Ok(self.0.add(user, date, id, amount).await?)
    }
    pub async fn remove(&self, user: Uuid, id: Uuid) -> Result<(), ApplicationError> {
        Ok(self.0.remove(user, id).await?)
    }
    pub async fn set_goal(
        &self,
        user: Uuid,
        date: NaiveDate,
        goal: i32,
    ) -> Result<DailyWater, ApplicationError> {
        if !(100..=10000).contains(&goal) {
            return Err(DomainError::Validation("water goal must be 100–10000 ml".into()).into());
        }
        Ok(self.0.set_goal(user, date, goal).await?)
    }
}
