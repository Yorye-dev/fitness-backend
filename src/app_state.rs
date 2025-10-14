use crate::repositories::*;
use crate::services::*;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub services: Arc<Services>,
    pub jwt_secret: String,
}

#[derive(Clone)]
pub struct Services {
    pub auth: AuthService,
    pub nutrition: NutritionService,
    pub workouts: WorkoutsService,
    pub progress: ProgressService,
}

impl Services {
    pub fn new(db: PgPool, jwt_secret: String) -> Self {
        Self {
            auth: AuthService::new(UserRepository::new(db.clone()), jwt_secret.clone()),
            nutrition: NutritionService::new(MealRepository::new(db.clone())),
            workouts: WorkoutsService::new(WorkoutRepository::new(db.clone())),
            progress: ProgressService::new(ProgressRepository::new(db.clone())),
        }
    }
}

