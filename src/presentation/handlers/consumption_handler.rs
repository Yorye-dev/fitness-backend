use axum::{
    extract::{State, Extension, Query},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use chrono::NaiveDate;
use crate::services::Services;
use crate::auth::claims::Claims;
use crate::domain::nutrition::repository::{ConsumptionRepository, MealRepository, NutritionRepository};
use crate::domain::nutrition::consumption::{DailyConsumption, DailySummary};
use crate::dtos::log_consumption_dto::LogConsumptionDto;
use crate::presentation::factories::api_response_factory::ResponseFactory;

#[derive(Deserialize)]
pub struct DailyProgressQuery {
    date: Option<String>,
}

pub async fn log_consumption_handler(
    State(services): State<Services>,
    Extension(claims): Extension<Claims>,
    Json(consumption_dto): Json<LogConsumptionDto>,
) -> impl IntoResponse {
    if let Err(errors) = consumption_dto.validate() {
        return ResponseFactory::bad_request(&errors.join(", "));
    }

    let user_id = match Uuid::parse_str(&claims.subject) {
        Ok(id) => id,
        Err(_) => return ResponseFactory::bad_request("Invalid user ID"),
    };

    let meal_id = match Uuid::parse_str(&consumption_dto.meal_id) {
        Ok(id) => id,
        Err(_) => return ResponseFactory::bad_request("Invalid meal_id format"),
    };

    let meal = match services.goals_repository.get_meal_by_id(&meal_id).await {
        Ok(Some(m)) => m,
        Ok(None) => return ResponseFactory::not_found("Meal not found"),
        Err(e) => return ResponseFactory::internal_error(&e.to_string()),
    };

    let date = match &consumption_dto.date {
        Some(d) => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return ResponseFactory::bad_request("Invalid date format. Use YYYY-MM-DD"),
        },
        None => chrono::Utc::now().date_naive(),
    };

    let quantity_factor = consumption_dto.quantity_grams / 100.0;
    let calories = meal.calories_per_100g * quantity_factor;
    let protein = meal.protein_per_100g * quantity_factor;
    let carbs = meal.carbs_per_100g * quantity_factor;
    let fat = meal.fat_per_100g * quantity_factor;

    let consumption = DailyConsumption {
        id: Uuid::new_v4(),
        user_id,
        date,
        meal_id,
        quantity_grams: consumption_dto.quantity_grams,
        calories_consumed: calories,
        protein_consumed: protein,
        carbs_consumed: carbs,
        fat_consumed: fat,
    };

    match services.goals_repository.log_consumption(&consumption).await {
        Ok(saved) => ResponseFactory::created(saved),
        Err(e) => ResponseFactory::internal_error(&e.to_string()),
    }
}

pub async fn daily_progress_handler(
    State(services): State<Services>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<DailyProgressQuery>,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&claims.subject) {
        Ok(id) => id,
        Err(_) => return ResponseFactory::bad_request("Invalid user ID"),
    };

    let date = match &query.date {
        Some(d) => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return ResponseFactory::bad_request("Invalid date format. Use YYYY-MM-DD"),
        },
        None => chrono::Utc::now().date_naive(),
    };

    let goals = match services.goals_repository.get_user_goals(&user_id).await {
        Ok(Some(g)) => g,
        Ok(None) => return ResponseFactory::not_found("Goals not found. Please set your nutrition goals first"),
        Err(e) => return ResponseFactory::internal_error(&e.to_string()),
    };

    let consumptions = match services.goals_repository.get_daily_consumption(&user_id, &date).await {
        Ok(c) => c,
        Err(e) => return ResponseFactory::internal_error(&e.to_string()),
    };

    let total_calories: f32 = consumptions.iter().map(|c| c.calories_consumed).sum();
    let total_protein: f32 = consumptions.iter().map(|c| c.protein_consumed).sum();
    let total_carbs: f32 = consumptions.iter().map(|c| c.carbs_consumed).sum();
    let total_fat: f32 = consumptions.iter().map(|c| c.fat_consumed).sum();

    let calc_percent = |consumed: f32, goal: f32| -> f32 {
        if goal > 0.0 {
            (consumed / goal) * 100.0
        } else {
            0.0
        }
    };

    let consumed = crate::domain::nutrition::consumption::MacroSummary {
        calories: total_calories,
        protein: total_protein,
        carbs: total_carbs,
        fat: total_fat,
    };

    let goals_summary = crate::domain::nutrition::consumption::MacroSummary {
        calories: goals.tdee,
        protein: goals.protein_goal,
        carbs: goals.carbs_goal,
        fat: goals.fats_goal,
    };

    let remaining = crate::domain::nutrition::consumption::MacroSummary {
        calories: goals.tdee - total_calories,
        protein: goals.protein_goal - total_protein,
        carbs: goals.carbs_goal - total_carbs,
        fat: goals.fats_goal - total_fat,
    };

    let avg_percent = (
        calc_percent(total_calories, goals.tdee) +
        calc_percent(total_protein, goals.protein_goal) +
        calc_percent(total_carbs, goals.carbs_goal) +
        calc_percent(total_fat, goals.fats_goal)
    ) / 4.0;

    let summary = crate::domain::nutrition::consumption::DailySummary {
        date: date.format("%Y-%m-%d").to_string(),
        consumed,
        goals: goals_summary,
        remaining,
        progress_percentage: avg_percent,
        macros: crate::domain::nutrition::consumption::MacroProgress {
            calories_percent: calc_percent(total_calories, goals.tdee),
            protein_percent: calc_percent(total_protein, goals.protein_goal),
            carbs_percent: calc_percent(total_carbs, goals.carbs_goal),
            fat_percent: calc_percent(total_fat, goals.fats_goal),
        },
    };

    ResponseFactory::ok(summary)
}

pub fn consumption_routes(services: Services) -> Router {
    Router::new()
        .route("/log", post(log_consumption_handler))
        .route("/progress", get(daily_progress_handler))
        .with_state(services)
}