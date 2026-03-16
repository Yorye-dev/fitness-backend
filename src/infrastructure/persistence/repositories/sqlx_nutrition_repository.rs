use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use chrono::NaiveDate;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::nutrition::meal::Meal;
use crate::domain::nutrition::consumption::DailyConsumption;
use crate::domain::nutrition::repository::{
    NutritionRepository as NutritionRepositoryTrait, 
    MealRepository as MealRepositoryTrait,
    ConsumptionRepository as ConsumptionRepositoryTrait,
    ConsumptionWithMeal, StatsSummary,
};

#[derive(Clone)]
pub struct SqlxNutritionRepository {
    pool: Pool<Postgres>,
}

impl SqlxNutritionRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NutritionRepositoryTrait for SqlxNutritionRepository {
    async fn get_user_goals(&self, user_id: &Uuid) -> Result<Option<NutritionGoals>, sqlx::Error> {
        let goals = sqlx::query_as::<_, NutritionGoals>(
            "SELECT id, user_id, protein_goal, carbs_goal, fats_goal, tdee, bmr
             FROM users_nutrition_goals WHERE user_id = $1"
        )
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(goals)
    }

    async fn save_user_goals(&self, goals: &NutritionGoals) -> Result<NutritionGoals, sqlx::Error> {
        let saved_goals = sqlx::query_as::<_, NutritionGoals>(
            "INSERT INTO users_nutrition_goals (id, user_id, protein_goal, carbs_goal, fats_goal, tdee, bmr)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING id, user_id, protein_goal, carbs_goal, fats_goal, tdee, bmr"
        )
            .bind(&goals.id)
            .bind(&goals.user_id)
            .bind(&goals.protein_goal)
            .bind(&goals.carbs_goal)
            .bind(&goals.fats_goal)
            .bind(&goals.tdee)
            .bind(&goals.bmr)
            .fetch_one(&self.pool)
            .await?;

        Ok(saved_goals)
    }

    async fn update_user_goals(&self, goals: &NutritionGoals) -> Result<NutritionGoals, sqlx::Error> {
        let updated_goals = sqlx::query_as::<_, NutritionGoals>(
            "UPDATE users_nutrition_goals 
             SET protein_goal = $1, carbs_goal = $2, fats_goal = $3, tdee = $4, bmr = $5
             WHERE user_id = $6
             RETURNING id, user_id, protein_goal, carbs_goal, fats_goal, tdee, bmr"
        )
            .bind(&goals.protein_goal)
            .bind(&goals.carbs_goal)
            .bind(&goals.fats_goal)
            .bind(&goals.tdee)
            .bind(&goals.bmr)
            .bind(&goals.user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(updated_goals)
    }
}

#[async_trait]
impl MealRepositoryTrait for SqlxNutritionRepository {
    async fn save_meal(&self, meal: &Meal) -> Result<Meal, sqlx::Error> {
        let saved_meal = sqlx::query_as::<_, Meal>(
            "INSERT INTO meals (id, user_id, name, calories_per_100g, protein_per_100g, carbs_per_100g, fat_per_100g)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING id, user_id, name, calories_per_100g, protein_per_100g, carbs_per_100g, fat_per_100g"
        )
            .bind(&meal.id)
            .bind(&meal.user_id)
            .bind(&meal.name)
            .bind(&meal.calories_per_100g)
            .bind(&meal.protein_per_100g)
            .bind(&meal.carbs_per_100g)
            .bind(&meal.fat_per_100g)
            .fetch_one(&self.pool)
            .await?;

        Ok(saved_meal)
    }

    async fn get_meal_by_id(&self, meal_id: &Uuid, user_id: &Uuid) -> Result<Option<Meal>, sqlx::Error> {
        let meal = sqlx::query_as::<_, Meal>(
            "SELECT id, user_id, name, calories_per_100g, protein_per_100g, carbs_per_100g, fat_per_100g
             FROM meals WHERE id = $1 AND user_id = $2"
        )
            .bind(meal_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(meal)
    }

    async fn get_all_meals(&self, user_id: &Uuid) -> Result<Vec<Meal>, sqlx::Error> {
        let meals = sqlx::query_as::<_, Meal>(
            "SELECT id, user_id, name, calories_per_100g, protein_per_100g, carbs_per_100g, fat_per_100g
             FROM meals WHERE user_id = $1 ORDER BY name"
        )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(meals)
    }

    async fn get_meals_paginated(&self, user_id: &Uuid, page: u32, per_page: u32) -> Result<(Vec<Meal>, i64), sqlx::Error> {
        let _offset = (page - 1) * per_page;
        
        let meals = sqlx::query_as::<_, Meal>(
            "SELECT id, user_id, name, calories_per_100g, protein_per_100g, carbs_per_100g, fat_per_100g
             FROM meals WHERE user_id = $1 ORDER BY name LIMIT $2 OFFSET $3"
        )
            .bind(user_id)
            .bind(per_page as i64)
            .bind(_offset as i64)
            .fetch_all(&self.pool)
            .await?;

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM meals WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok((meals, total.0))
    }

    async fn delete_meal(&self, meal_id: &Uuid, user_id: &Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM meals WHERE id = $1 AND user_id = $2")
            .bind(meal_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[async_trait]
impl ConsumptionRepositoryTrait for SqlxNutritionRepository {
    async fn log_consumption(&self, consumption: &DailyConsumption) -> Result<DailyConsumption, sqlx::Error> {
        let saved = sqlx::query_as::<_, DailyConsumption>(
            "INSERT INTO daily_consumption (id, user_id, date, meal_id, quantity_grams, calories_consumed, protein_consumed, carbs_consumed, fat_consumed)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING id, user_id, date, meal_id, quantity_grams, calories_consumed, protein_consumed, carbs_consumed, fat_consumed"
        )
            .bind(&consumption.id)
            .bind(&consumption.user_id)
            .bind(&consumption.date)
            .bind(&consumption.meal_id)
            .bind(&consumption.quantity_grams)
            .bind(&consumption.calories_consumed)
            .bind(&consumption.protein_consumed)
            .bind(&consumption.carbs_consumed)
            .bind(&consumption.fat_consumed)
            .fetch_one(&self.pool)
            .await?;

        Ok(saved)
    }

    async fn get_daily_consumption(&self, user_id: &Uuid, date: &NaiveDate) -> Result<Vec<DailyConsumption>, sqlx::Error> {
        let consumptions = sqlx::query_as::<_, DailyConsumption>(
            "SELECT id, user_id, date, meal_id, quantity_grams, calories_consumed, protein_consumed, carbs_consumed, fat_consumed
             FROM daily_consumption WHERE user_id = $1 AND date = $2"
        )
            .bind(user_id)
            .bind(date)
            .fetch_all(&self.pool)
            .await?;

        Ok(consumptions)
    }

    async fn delete_consumption(&self, consumption_id: &Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM daily_consumption WHERE id = $1")
            .bind(consumption_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_consumptions_paginated(
        &self, 
        user_id: &Uuid, 
        page: u32, 
        per_page: u32, 
        start_date: Option<NaiveDate>, 
        end_date: Option<NaiveDate>
    ) -> Result<(Vec<ConsumptionWithMeal>, i64), sqlx::Error> {
        let _offset = (page - 1) * per_page;

        let mut query = String::from(
            "SELECT dc.id, dc.user_id, dc.date, dc.meal_id, dc.quantity_grams, 
                    dc.calories_consumed, dc.protein_consumed, dc.carbs_consumed, dc.fat_consumed,
                    m.name as meal_name, m.calories_per_100g, m.protein_per_100g, m.carbs_per_100g, m.fat_per_100g
             FROM daily_consumption dc
             JOIN meals m ON dc.meal_id = m.id
             WHERE dc.user_id = $1"
        );

        let mut count_query = "SELECT COUNT(*) FROM daily_consumption dc WHERE dc.user_id = $1".to_string();
        let mut param_idx = 2;

        if let Some(_start) = start_date {
            query.push_str(&format!(" AND dc.date >= ${}", param_idx));
            count_query.push_str(&format!(" AND dc.date >= ${}", param_idx));
            param_idx += 1;
        }
        if let Some(_end) = end_date {
            query.push_str(&format!(" AND dc.date <= ${}", param_idx));
            count_query.push_str(&format!(" AND dc.date <= ${}", param_idx));
            param_idx += 1;
        }

        query.push_str(&format!(" ORDER BY dc.date DESC, dc.id LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

        let mut rows = sqlx::query(&query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let consumptions: Vec<ConsumptionWithMeal> = rows.iter_mut().map(|row| {
            ConsumptionWithMeal {
                consumption: DailyConsumption {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    date: row.get("date"),
                    meal_id: row.get("meal_id"),
                    quantity_grams: row.get("quantity_grams"),
                    calories_consumed: row.get("calories_consumed"),
                    protein_consumed: row.get("protein_consumed"),
                    carbs_consumed: row.get("carbs_consumed"),
                    fat_consumed: row.get("fat_consumed"),
                },
                meal_name: row.get("meal_name"),
                meal_calories: row.get("calories_per_100g"),
                meal_protein: row.get("protein_per_100g"),
                meal_carbs: row.get("carbs_per_100g"),
                meal_fat: row.get("fat_per_100g"),
            }
        }).collect();

        let mut count_rows = sqlx::query(&count_query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let total: i64 = count_rows.iter_mut().map(|row| row.get("count")).next().unwrap_or(0);

        Ok((consumptions, total))
    }

    async fn get_consumptions_by_date_range(
        &self, 
        user_id: &Uuid, 
        start_date: &NaiveDate, 
        end_date: &NaiveDate
    ) -> Result<Vec<ConsumptionWithMeal>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT dc.id, dc.user_id, dc.date, dc.meal_id, dc.quantity_grams, 
                    dc.calories_consumed, dc.protein_consumed, dc.carbs_consumed, dc.fat_consumed,
                    m.name as meal_name, m.calories_per_100g, m.protein_per_100g, m.carbs_per_100g, m.fat_per_100g
             FROM daily_consumption dc
             JOIN meals m ON dc.meal_id = m.id
             WHERE dc.user_id = $1 AND dc.date >= $2 AND dc.date <= $3
             ORDER BY dc.date DESC, dc.id"
        )
            .bind(user_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&self.pool)
            .await?;

        let consumptions: Vec<ConsumptionWithMeal> = rows.iter().map(|row| {
            ConsumptionWithMeal {
                consumption: DailyConsumption {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    date: row.get("date"),
                    meal_id: row.get("meal_id"),
                    quantity_grams: row.get("quantity_grams"),
                    calories_consumed: row.get("calories_consumed"),
                    protein_consumed: row.get("protein_consumed"),
                    carbs_consumed: row.get("carbs_consumed"),
                    fat_consumed: row.get("fat_consumed"),
                },
                meal_name: row.get("meal_name"),
                meal_calories: row.get("calories_per_100g"),
                meal_protein: row.get("protein_per_100g"),
                meal_carbs: row.get("carbs_per_100g"),
                meal_fat: row.get("fat_per_100g"),
            }
        }).collect();

        Ok(consumptions)
    }

    async fn get_date_range_stats(
        &self, 
        user_id: &Uuid, 
        start_date: &NaiveDate, 
        end_date: &NaiveDate
    ) -> Result<StatsSummary, sqlx::Error> {
        let row = sqlx::query(
            "SELECT 
                COUNT(DISTINCT date) as total_days,
                SUM(calories_consumed) as total_calories,
                SUM(protein_consumed) as total_protein,
                SUM(carbs_consumed) as total_carbs,
                SUM(fat_consumed) as total_fat
             FROM daily_consumption 
             WHERE user_id = $1 AND date >= $2 AND date <= $3"
        )
            .bind(user_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_one(&self.pool)
            .await?;

        let total_days: i32 = row.get("total_days");
        let total_calories: f32 = row.get("total_calories");
        let total_protein: f32 = row.get("total_protein");
        let total_carbs: f32 = row.get("total_carbs");
        let total_fat: f32 = row.get("total_fat");

        let days = total_days as f32;
        Ok(StatsSummary {
            total_days,
            avg_calories: if days > 0.0 { total_calories / days } else { 0.0 },
            avg_protein: if days > 0.0 { total_protein / days } else { 0.0 },
            avg_carbs: if days > 0.0 { total_carbs / days } else { 0.0 },
            avg_fat: if days > 0.0 { total_fat / days } else { 0.0 },
            total_calories,
            total_protein,
            total_carbs,
            total_fat,
        })
    }
}
