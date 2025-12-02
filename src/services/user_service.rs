use axum::Extension;
use tracing_subscriber::util;

use crate::models::user;
use crate::repositories::user_repository::{self, UserRepository};
use crate::factories::user_factory::UserFactory;
use crate::dtos::register_user_dto::RegisterUserDto;
use crate::repositories::user_nutrition_goals::GoalsRepository;
use crate::auth::claims::Claims;
use crate::models::user_nutrition_goals::UserNutritionsGoals;

use crate::utils;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    goals_repo: GoalsRepository,
}

impl UserService {

    pub fn new(user_repo: UserRepository, goals_repo: GoalsRepository) -> Self {
        
        Self { user_repo, goals_repo }
    }

    pub async fn register_user(&self, dto : RegisterUserDto) {

        let user = UserFactory::create_user_from_dto(dto).unwrap();

        let _ =self.user_repo.save_user(&user).await;
    }

    pub async fn get_daily_stats(&self, claims: Claims) ->  Result<UserNutritionsGoals, String> {

        // Orquestador
        
        let user_id = utils::parsers::parse_uuid(&claims.subject)?;

        let user = self.user_repo.get_user_by_id(&user_id);



        // let goals = self.goals_repo.get_daily_stats(user_id).await;
        // let today_meals = self.meals_repo.find_today_by_user_id(&user_id).await?;
        

    }

    // pub async fn grate_goals

    //pub async fn me (Extension(claims) :Extension<Claims>) -> impl IntoResponse {
}
