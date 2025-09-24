use sqlx::PgPool;

 pub mod user_service;
 pub mod auth_service;

use crate::services::user_service::UserService;
use crate::repositories::user_repository::UserRepository;
use crate::services::auth_service::AuthService;

#[derive(Clone)]
pub struct Services {
    pub user_service: UserService,
    pub auth_service: AuthService,
    //pub user_repository: UserRepository , otroservicio 

}

impl Services {
    pub fn new (pool: PgPool) -> Self {
        let user_repository = UserRepository::new(pool.clone());
        //mas repositories
        // Esto es correcto?
        Self { 
            user_service: UserService::new(user_repository.clone()),
            auth_service: AuthService::new(user_repository.clone())
        }
    }
}
