use axum::{
    extract::{State, Extension},
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use uuid::Uuid;
use crate::services::Services;
use crate::auth::claims::Claims;
use crate::domain::user::repository::UserRepository;
use crate::domain::nutrition::repository::NutritionRepository;
use crate::dtos::change_password_dto::ChangePasswordDto;
use crate::dtos::update_goals_dto::UpdateGoalsDto;
use crate::dtos::update_user_dto::UpdateUserDto;
use crate::presentation::factories::api_response_factory::ResponseFactory;

pub async fn get_goals_handler(
    State(services): State<Services>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&claims.subject) {
        Ok(id) => id,
        Err(_) => return ResponseFactory::bad_request("Invalid user ID"),
    };

    match services.goals_repository.get_user_goals(&user_id).await {
        Ok(Some(goals)) => ResponseFactory::ok(goals),
        Ok(None) => ResponseFactory::not_found("Goals not found"),
        Err(e) => ResponseFactory::internal_error(&e.to_string()),
    }
}

pub async fn update_goals_handler(
    State(services): State<Services>,
    Extension(claims): Extension<Claims>,
    Json(goals_dto): Json<UpdateGoalsDto>,
) -> impl IntoResponse {
    if let Err(errors) = goals_dto.validate() {
        return ResponseFactory::bad_request(&errors.join(", "));
    }

    let user_id = match Uuid::parse_str(&claims.subject) {
        Ok(id) => id,
        Err(_) => return ResponseFactory::bad_request("Invalid user ID"),
    };

    let user = match services.user_repository.get_user_by_id(&user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => return ResponseFactory::not_found("User not found"),
        Err(e) => return ResponseFactory::internal_error(&e.to_string()),
    };

    let tdee = goals_dto.protein_goal * 4.0 + goals_dto.carbs_goal * 4.0 + goals_dto.fats_goal * 9.0;
    let bmr = tdee / user.activity_level.multiplier();

    let goals = crate::domain::nutrition::goals::NutritionGoals::new(
        user_id,
        goals_dto.protein_goal,
        goals_dto.fats_goal,
        goals_dto.carbs_goal,
        tdee,
        bmr,
    );

    match services.goals_repository.update_user_goals(&goals).await {
        Ok(saved_goals) => ResponseFactory::ok(saved_goals),
        Err(e) => ResponseFactory::internal_error(&e.to_string()),
    }
}

pub async fn change_password_handler(
    State(services): State<Services>,
    Extension(claims): Extension<Claims>,
    Json(password_dto): Json<ChangePasswordDto>,
) -> impl IntoResponse {
    if let Err(errors) = password_dto.validate() {
        return ResponseFactory::bad_request(&errors.join(", "));
    }

    let user_id = match Uuid::parse_str(&claims.subject) {
        Ok(id) => id,
        Err(_) => return ResponseFactory::bad_request("Invalid user ID"),
    };

    match services.change_password_use_case.execute(
        user_id,
        password_dto.current_password,
        password_dto.new_password,
    ).await {
        Ok(true) => ResponseFactory::ok(serde_json::json!({ "message": "Password updated successfully" })),
        Ok(false) => ResponseFactory::not_found("User not found"),
        Err(e) => ResponseFactory::unauthorized(&e.to_string()),
    }
}

pub async fn update_user_handler(
    State(services): State<Services>,
    Extension(claims): Extension<Claims>,
    Json(user_dto): Json<UpdateUserDto>,
) -> impl IntoResponse {
    if let Err(errors) = user_dto.validate() {
        return ResponseFactory::bad_request(&errors.join(", "));
    }

    let user_id = match Uuid::parse_str(&claims.subject) {
        Ok(id) => id,
        Err(_) => return ResponseFactory::bad_request("Invalid user ID"),
    };

    match services.update_user_use_case.execute(user_id, user_dto).await {
        Ok((user, goals)) => ResponseFactory::ok(serde_json::json!({
            "user": user,
            "goals": goals
        })),
        Err(e) => match e {
            crate::domain::errors::DomainError::UserNotFound => ResponseFactory::not_found("User not found"),
            crate::domain::errors::DomainError::ValidationError(error) => ResponseFactory::bad_request(&error),
            _ => ResponseFactory::internal_error(&e.to_string()),
        },
    }
}

pub fn user_routes(services: Services) -> Router {
    Router::new()
        .route("/goals", get(get_goals_handler))
        .route("/goals", put(update_goals_handler))
        .route("/change-password", put(change_password_handler))
        .route("/user", put(update_user_handler))
        .with_state(services)
}