use crate::repositories::user_nutrition_goals::GoalsRepository;
use crate::repositories::user_repository::UserRepository;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    goals_repo: GoalsRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository, goals_repo: GoalsRepository) -> Self {
        Self {
            user_repo,
            goals_repo,
        }
    }
}
