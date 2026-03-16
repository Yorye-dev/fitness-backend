use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::NaiveDate;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::nutrition::meal::Meal;
use crate::domain::nutrition::consumption::DailyConsumption;
use crate::domain::nutrition::repository::{
    NutritionRepository as NutritionRepositoryTrait, 
    MealRepository as MealRepositoryTrait,
    ConsumptionRepository as ConsumptionRepositoryTrait,
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
            "INSERT INTO meals (id, name, calories_per_100g, protein_per_100g, carbs_per_100g, fat_per_100g)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, name, calories_per_100g, protein_per_100g, carbs_per_100g, fat_per_100g"
        )
            .bind(&meal.id)
            .bind(&meal.name)
            .bind(&meal.calories_per_100g)
            .bind(&meal.protein_per_100g)
            .bind(&meal.carbs_per_100g)
            .bind(&meal.fat_per_100g)
            .fetch_one(&self.pool)
            .await?;

        Ok(saved_meal)
    }

    async fn get_meal_by_id(&self, meal_id: &Uuid) -> Result<Option<Meal>, sqlx::Error> {
        let meal = sqlx::query_as::<_, Meal>(
            "SELECT id, name, calories_per_100g, protein_per_100g, carbs_per_100g, fat_per_100g
             FROM meals WHERE id = $1"
        )
            .bind(meal_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(meal)
    }

    async fn get_all_meals(&self) -> Result<Vec<Meal>, sqlx::Error> {
        let meals = sqlx::query_as::<_, Meal>(
            "SELECT id, name, calories_per_100g, protein_per_100g, carbs_per_100g, fat_per_100g
             FROM meals ORDER BY name"
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(meals)
    }

    async fn delete_meal(&self, meal_id: &Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM meals WHERE id = $1")
            .bind(meal_id)
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
}
