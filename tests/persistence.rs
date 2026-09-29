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
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn database_rejects_invalid_profiles(pool: PgPool) {
    let app = app(pool.clone());
    let account = register(&app, "alice").await;
    let error = sqlx::query("UPDATE users SET age=0 WHERE id=$1")
        .bind(account.id)
        .execute(&pool)
        .await
        .unwrap_err();
    assert_eq!(
        error.as_database_error().unwrap().code().as_deref(),
        Some("23514")
    );
    let (status, body) = request(&app, "GET", "/api/me", Some(&account.access), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["age"], 30);
}

#[sqlx::test]
async fn invalid_returned_profiles_roll_back_registration(pool: PgPool) {
    sqlx::raw_sql("CREATE FUNCTION invalidate_profile() RETURNS trigger LANGUAGE plpgsql AS
        $$ BEGIN NEW.weight := 301; RETURN NEW; END $$;
        CREATE TRIGGER invalidate_profile BEFORE INSERT ON users FOR EACH ROW EXECUTE FUNCTION invalidate_profile();")
        .execute(&pool).await.unwrap();
    let app = app(pool.clone());
    let (status, body) = request(
        &app,
        "POST",
        "/auth/register",
        None,
        Some(registration("alice")),
    )
    .await;
    assert_error(status, &body, StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL");
    for query in [
        "SELECT COUNT(*) FROM users",
        "SELECT COUNT(*) FROM nutrition_goal_versions",
    ] {
        let count: i64 = sqlx::query_scalar(query).fetch_one(&pool).await.unwrap();
        assert_eq!(count, 0);
    }
}

#[sqlx::test]
async fn failed_registration_rolls_back_and_hides_database_details(pool: PgPool) {
    sqlx::raw_sql("CREATE FUNCTION reject_goals() RETURNS trigger LANGUAGE plpgsql AS
        $$ BEGIN RAISE EXCEPTION 'private_database_detail'; END $$;
        CREATE TRIGGER reject_goals BEFORE INSERT ON nutrition_goal_versions FOR EACH ROW EXECUTE FUNCTION reject_goals();")
        .execute(&pool).await.unwrap();
    let app = app(pool.clone());
    let (status, body) = request(
        &app,
        "POST",
        "/auth/register",
        None,
        Some(registration("alice")),
    )
    .await;
    assert_error(status, &body, StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL");
    assert!(!body.to_string().contains("private_database_detail"));
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}
#[sqlx::test]
async fn persistence_handles_empty_stats_pagination_and_ownership(pool: PgPool) {
    let app = app(pool.clone());
    let alice = register(&app, "alice").await;
    let bob = register(&app, "bob").await;
    let repo = SqlxNutritionRepository::new(pool);
    let date = NaiveDate::from_ymd_opt(2026, 9, 17).unwrap();
    let empty = repo
        .get_date_range_stats(&alice.id, &date, &date)
        .await
        .unwrap();
    assert_eq!(empty.total_days, 0);
    assert_eq!(empty.total_calories, 0.0);
    assert_eq!(empty.avg_calories, 0.0);
    let meal = repo
        .save_meal(&Meal {
            id: Uuid::new_v4(),
            user_id: alice.id,
            name: "Test".into(),
            calories_per_100g: 100.0,
            protein_per_100g: 10.0,
            carbs_per_100g: 10.0,
            fat_per_100g: 2.0,
        })
        .await
        .unwrap();
    let consumption = DailyConsumption {
        id: Uuid::new_v4(),
        user_id: alice.id,
        date,
        meal_id: meal.id,
        quantity_grams: 100.0,
        calories_consumed: 100.0,
        protein_consumed: 10.0,
        carbs_consumed: 10.0,
        fat_consumed: 2.0,
    };
    repo.log_consumption(&consumption).await.unwrap();
    let (entries, total) = repo
        .get_consumptions_paginated(&alice.id, 1, 1, Some(date), Some(date))
        .await
        .unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(total, 1);
    assert_eq!(entries[0].meal_name, "Test");
    let (entries, total) = repo
        .get_consumptions_paginated(&alice.id, 2, 1, None, None)
        .await
        .unwrap();
    assert!(entries.is_empty());
    assert_eq!(total, 1);
    assert!(
        !repo
            .delete_consumption(&consumption.id, &bob.id)
            .await
            .unwrap()
    );
    let stolen = DailyConsumption {
        id: Uuid::new_v4(),
        user_id: bob.id,
        ..consumption.clone()
    };
    assert!(repo.log_consumption(&stolen).await.is_err());
    let stats = repo
        .get_date_range_stats(&alice.id, &date, &date)
        .await
        .unwrap();
    assert_eq!(stats.total_days, 1);
    assert_eq!(stats.total_calories, 100.0);
    assert!(
        repo.delete_consumption(&consumption.id, &alice.id)
            .await
            .unwrap()
    );
}
