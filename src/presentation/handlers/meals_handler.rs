use axum::{
    extract::{State, Extension},
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};
use uuid::Uuid;
use crate::services::Services;
use crate::auth::claims::Claims;
use crate::domain::nutrition::meal::Meal;
use crate::domain::nutrition::repository::MealRepository;
use crate::dtos::create_meal_dto::CreateMealDto;
use crate::presentation::factories::api_response_factory::ResponseFactory;

pub async fn create_meal_handler(
    State(services): State<Services>,
    Extension(_claims): Extension<Claims>,
    Json(meal_dto): Json<CreateMealDto>,
) -> impl IntoResponse {
    if let Err(errors) = meal_dto.validate() {
        return ResponseFactory::bad_request(&errors.join(", "));
    }

    let meal = Meal {
        id: Uuid::new_v4(),
        name: meal_dto.name,
        calories_per_100g: meal_dto.calories_per_100g,
        protein_per_100g: meal_dto.protein_per_100g,
        carbs_per_100g: meal_dto.carbs_per_100g,
        fat_per_100g: meal_dto.fat_per_100g,
    };

    match services.goals_repository.save_meal(&meal).await {
        Ok(saved_meal) => ResponseFactory::created(saved_meal),
        Err(e) => ResponseFactory::internal_error(&e.to_string()),
    }
}

pub async fn get_meals_handler(
    State(services): State<Services>,
    Extension(_claims): Extension<Claims>,
) -> impl IntoResponse {
    match services.goals_repository.get_all_meals().await {
        Ok(meals) => ResponseFactory::ok(meals),
        Err(e) => ResponseFactory::internal_error(&e.to_string()),
    }
}

pub async fn delete_meal_handler(
    State(services): State<Services>,
    Extension(_claims): Extension<Claims>,
    Json(meal_id): Json<serde_json::Value>,
) -> impl IntoResponse {
    let meal_id_str = meal_id.get("id")
        .and_then(|v| v.as_str())
        .ok_or("Missing or invalid meal id");

    match meal_id_str {
        Ok(id_str) => {
            match Uuid::parse_str(id_str) {
                Ok(id) => {
                    match services.goals_repository.delete_meal(&id).await {
                        Ok(true) => ResponseFactory::ok(serde_json::json!({ "message": "Meal deleted" })),
                        Ok(false) => ResponseFactory::not_found("Meal not found"),
                        Err(e) => ResponseFactory::internal_error(&e.to_string()),
                    }
                },
                Err(_) => ResponseFactory::bad_request("Invalid UUID format"),
            }
        },
        Err(msg) => ResponseFactory::bad_request(msg),
    }
}

pub fn meals_routes(services: Services) -> Router {
    Router::new()
        .route("/meals", post(create_meal_handler))
        .route("/meals", get(get_meals_handler))
        .route("/meals", delete(delete_meal_handler))
        .with_state(services)
}