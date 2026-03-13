use crate::domain::nutrition::repository::NutritionRepository as NutritionRepositoryTrait;
use crate::domain::user::repository::UserRepository as UserRepositoryTrait;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<dyn UserRepositoryTrait>,
    goals_repo: Arc<dyn NutritionRepositoryTrait>,
}

impl UserService {
    pub fn new(
        user_repo: Arc<dyn UserRepositoryTrait>,
        goals_repo: Arc<dyn NutritionRepositoryTrait>,
    ) -> Self {
        Self {
            user_repo,
            goals_repo,
        }
    }
}
