use sqlx::PgPool;

 pub mod user_service;
 pub mod auth_service;

use crate::services::user_service::UserService;
use crate::repositories::user_repository::UserRepository;


#[derive(Clone)]
pub struct Services {
    pub user_service: UserService,
    //pub user_repository: UserRepository , otroservicio 

}

impl Services {
    pub fn new (pool: PgPool) -> Self {
        let user_repository = UserRepository::new(pool.clone());
        //mas repositories
        
        Self { 
            user_service: UserService::new(user_repository) 
        }
    }
}
