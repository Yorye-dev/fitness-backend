use sqlx::PgPool;
use std::sync::Arc;

 pub mod user_service;
 pub mod auth_service;
 pub mod errors;

use crate::infrastructure::persistence::repositories::SqlxUserRepository;
use crate::infrastructure::persistence::repositories::SqlxNutritionRepository;
use crate::services::user_service::UserService;
use crate::services::auth_service::AuthService;
use crate::domain::user::repository::UserRepository as UserRepositoryTrait;
use crate::domain::nutrition::repository::NutritionRepository as NutritionRepositoryTrait;
use crate::auth::jwt::Jwt;

#[derive(Clone)]
pub struct Services {
    pub user_service: UserService,
    pub auth_service: AuthService,
}

impl Services {
    pub fn new (pool: PgPool, secret_key: String) -> Self {
        
        let jwt = Jwt::new(secret_key);

        let user_repository: Arc<dyn UserRepositoryTrait> = Arc::new(SqlxUserRepository::new(pool.clone()));
        let goals_repository: Arc<dyn NutritionRepositoryTrait> = Arc::new(SqlxNutritionRepository::new(pool.clone()));
        
        Self { 
            user_service: UserService::new(user_repository.clone(), goals_repository.clone()),
            auth_service: AuthService::new(user_repository.clone(), goals_repository.clone(), jwt)
        }
    }
}
