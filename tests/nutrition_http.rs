mod common;
use axum::http::StatusCode;
use chrono::NaiveDate;
use common::*;
use fitness_backend::{
    domain::nutrition::{
        consumption::DailyConsumption,
        meal::Meal,
        repository::{ConsumptionRepository, MealRepository},
    },
    infrastructure::persistence::repositories::SqlxNutritionRepository,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

async fn seed(
    pool: &PgPool,
    user_id: Uuid,
    date: &str,
    calories: f32,
    protein: f32,
    carbs: f32,
    fat: f32,
) {
    let repository = SqlxNutritionRepository::new(pool.clone());
    let meal = repository
        .save_meal(&Meal {
            id: Uuid::new_v4(),
            user_id,
            name: "Oats".into(),
            calories_per_100g: calories,
            protein_per_100g: protein,
            carbs_per_100g: carbs,
            fat_per_100g: fat,
        })
        .await
        .unwrap();
    repository
        .log_consumption(&DailyConsumption {
            id: Uuid::new_v4(),
            user_id,
            date: NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
            meal_id: meal.id,
            quantity_grams: 100.0,
            calories_consumed: calories,
            protein_consumed: protein,
            carbs_consumed: carbs,
            fat_consumed: fat,
        })
        .await
        .unwrap();
}
#[sqlx::test]
async fn empty_day_is_success_with_backend_targets(pool: PgPool) {
    let app = app(pool);
    let account = register(&app, "alice").await;
    let (status, body) = request(
        &app,
        "GET",
        "/api/nutrition/daily?date=2026-09-17",
        Some(&account.access),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["date"], "2026-09-17");
    assert_eq!(body["data"]["meals"], json!([]));
    assert_eq!(body["data"]["calories"]["consumed"], 0.0);
    assert!(body["data"]["calories"]["target"].as_f64().unwrap() > 0.0);
    for nutrient in ["protein", "carbs", "fat"] {
        assert_eq!(body["data"]["macros"][nutrient]["consumed"], 0.0);
        assert!(body["data"]["macros"][nutrient]["target"].as_f64().unwrap() > 0.0);
    }
}
#[sqlx::test]
async fn daily_totals_respect_date_user_and_saved_targets(pool: PgPool) {
    let app = app(pool.clone());
    let alice = register(&app, "alice").await;
    let bob = register(&app, "bob").await;
    seed(&pool, alice.id, "2026-09-17", 100.0, 10.5, 12.0, 2.0).await;
    seed(&pool, alice.id, "2026-09-17", 200.0, 20.0, 25.0, 4.0).await;
    seed(&pool, alice.id, "2026-09-18", 900.0, 90.0, 90.0, 90.0).await;
    seed(&pool, bob.id, "2026-09-17", 800.0, 80.0, 80.0, 80.0).await;
    let (status, _) = request(
        &app,
        "PUT",
        "/api/nutrition/goals",
        Some(&alice.access),
        Some(json!({"tdee":2300.0,"protein_goal":160.0,"carbs_goal":250.0,"fats_goal":70.0})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, body) = request(
        &app,
        "GET",
        "/api/nutrition/daily?date=2026-09-17",
        Some(&alice.access),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["meals"].as_array().unwrap().len(), 2);
    assert_eq!(
        body["data"]["calories"],
        json!({"consumed":300.0,"target":2300.0})
    );
    assert_eq!(
        body["data"]["macros"]["protein"],
        json!({"consumed":30.5,"target":160.0})
    );
    assert_eq!(
        body["data"]["macros"]["carbs"],
        json!({"consumed":37.0,"target":250.0})
    );
    assert_eq!(
        body["data"]["macros"]["fat"],
        json!({"consumed":6.0,"target":70.0})
    );
}
#[sqlx::test]
async fn daily_rejects_bad_dates_and_user_id_parameters(pool: PgPool) {
    let app = app(pool);
    let account = register(&app, "alice").await;
    for query in [
        "",
        "?date=bad",
        "?date=2026-02-30",
        "?date=2026-9-1",
        "?date=2026-09-17&user_id=other",
        "?date=2026-09-17&date=2026-09-18",
    ] {
        let (status, body) = request(
            &app,
            "GET",
            &format!("/api/nutrition/daily{query}"),
            Some(&account.access),
            None,
        )
        .await;
        assert_error(
            status,
            &body,
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
        );
    }
    let (status, _) = request(
        &app,
        "GET",
        "/api/nutrition/daily?date=2024-02-29",
        Some(&account.access),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
}
#[sqlx::test]
async fn daily_can_calculate_missing_goals(pool: PgPool) {
    let app = app(pool.clone());
    let account = register(&app, "alice").await;
    sqlx::query("DELETE FROM users_nutrition_goals WHERE user_id=$1")
        .bind(account.id)
        .execute(&pool)
        .await
        .unwrap();
    let (status, body) = request(
        &app,
        "GET",
        "/api/nutrition/daily?date=2026-09-17",
        Some(&account.access),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["data"]["calories"]["target"].as_f64().unwrap() > 0.0);
    assert_eq!(body["data"]["meals"], json!([]));
}
#[sqlx::test]
async fn meal_crud_is_scoped_and_paginated(pool: PgPool) {
    let app = app(pool);
    let alice = register(&app, "alice").await;
    let bob = register(&app, "bob").await;
    let meal = json!({"name":"Oats","calories_per_100g":380.0,"protein_per_100g":13.0,"carbs_per_100g":60.0,"fat_per_100g":7.0});
    let (status, created) = request(
        &app,
        "POST",
        "/api/nutrition/meals",
        Some(&alice.access),
        Some(meal.clone()),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let path = format!(
        "/api/nutrition/meals/{}",
        created["data"]["id"].as_str().unwrap()
    );
    for method in ["GET", "PUT", "DELETE"] {
        let data = if method == "PUT" {
            Some(meal.clone())
        } else {
            None
        };
        let (status, body) = request(&app, method, &path, Some(&bob.access), data).await;
        assert_error(status, &body, StatusCode::NOT_FOUND, "MEAL_NOT_FOUND");
    }
    let (status, body) = request(
        &app,
        "GET",
        "/api/nutrition/meals?page=1&per_page=1",
        Some(&alice.access),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["meta"]["pagination"]["total"], 1);
    let (status, _) = request(
        &app,
        "GET",
        "/api/nutrition/meals?page=0",
        Some(&alice.access),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, _) = request(&app, "PUT", &path, Some(&alice.access), Some(meal)).await;
    assert_eq!(status, StatusCode::OK);
    let (status, _) = request(&app, "DELETE", &path, Some(&alice.access), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
}
