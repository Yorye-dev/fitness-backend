use super::goals::{GOAL_COLUMNS, save_current_goals};
use crate::domain::{
    errors::RepositoryError,
    nutrition::{
        consumption::DailyConsumption,
        goals::NutritionGoals,
        meal::Meal,
        repository::{
            ConsumptionRepository, ConsumptionWithMeal, MealRepository, NutritionRepository,
            StatsSummary,
        },
    },
};
use crate::infrastructure::persistence::models::nutrition_row::{
    ConsumptionRow, ConsumptionWithMealRow, MealRow, NutritionGoalsRow, StatsRow,
};
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

// SQL performs all storage/aggregation arithmetic in NUMERIC. These explicit projections
// preserve the existing application and JSON contracts until their decimal-type migration.
const FOOD_COLUMNS: &str = "id,user_id,name,calories_per_100g::real AS calories_per_100g,
    protein_per_100g::real AS protein_per_100g,carbs_per_100g::real AS carbs_per_100g,
    fat_per_100g::real AS fat_per_100g";
const CONSUMPTION_COLUMNS: &str = "id,user_id,date,meal_id,quantity_grams::real AS quantity_grams,
    calories_consumed::real AS calories_consumed,protein_consumed::real AS protein_consumed,
    carbs_consumed::real AS carbs_consumed,fat_consumed::real AS fat_consumed";
const ENTRY_COLUMNS: &str = "id,user_id,date,meal_id,quantity_grams::real AS quantity_grams,
    calories_consumed::real AS calories_consumed,protein_consumed::real AS protein_consumed,
    carbs_consumed::real AS carbs_consumed,fat_consumed::real AS fat_consumed,meal_name,
    calories_per_100g::real AS calories_per_100g,protein_per_100g::real AS protein_per_100g,
    carbs_per_100g::real AS carbs_per_100g,fat_per_100g::real AS fat_per_100g";

#[derive(Clone)]
pub struct SqlxNutritionRepository {
    pool: PgPool,
}
impl SqlxNutritionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NutritionRepository for SqlxNutritionRepository {
    async fn get_daily_meals(
        &self,
        user_id: &Uuid,
        date: &NaiveDate,
    ) -> Result<Vec<ConsumptionWithMeal>, RepositoryError> {
        let rows = sqlx::query_as::<_, ConsumptionWithMealRow>(&format!(
            "SELECT {ENTRY_COLUMNS} FROM consumption_entries WHERE user_id=$1 AND date=$2 ORDER BY id"
        ))
        .bind(user_id).bind(date).fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn get_user_goals(
        &self,
        user_id: &Uuid,
    ) -> Result<Option<NutritionGoals>, RepositoryError> {
        Ok(sqlx::query_as::<_, NutritionGoalsRow>(&format!(
            "SELECT {GOAL_COLUMNS} FROM nutrition_goal_versions
             WHERE user_id=$1 AND effective_from <=
               (SELECT (CURRENT_TIMESTAMP AT TIME ZONE time_zone)::date FROM users WHERE id=$1)
             ORDER BY effective_from DESC LIMIT 1"
        ))
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn get_user_goals_on_date(
        &self,
        user_id: &Uuid,
        date: &NaiveDate,
    ) -> Result<Option<NutritionGoals>, RepositoryError> {
        Ok(sqlx::query_as::<_, NutritionGoalsRow>(&format!(
            "SELECT {GOAL_COLUMNS} FROM nutrition_goal_versions
             WHERE user_id=$1 AND effective_from <= $2 ORDER BY effective_from DESC LIMIT 1"
        ))
        .bind(user_id)
        .bind(date)
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn save_user_goals(
        &self,
        goals: &NutritionGoals,
    ) -> Result<NutritionGoals, RepositoryError> {
        let mut transaction = self.pool.begin().await?;
        let saved = save_current_goals(&mut transaction, goals, "manual").await?;
        transaction.commit().await?;
        Ok(saved)
    }

    async fn update_user_goals(
        &self,
        goals: &NutritionGoals,
    ) -> Result<NutritionGoals, RepositoryError> {
        self.save_user_goals(goals).await
    }
}

#[async_trait]
impl MealRepository for SqlxNutritionRepository {
    async fn save_meal(&self, meal: &Meal) -> Result<Meal, RepositoryError> {
        Ok(sqlx::query_as::<_, MealRow>(&format!(
            "INSERT INTO foods (id,user_id,name,calories_per_100g,protein_per_100g,carbs_per_100g,fat_per_100g)
             VALUES ($1,$2,$3,$4::text::numeric,$5::text::numeric,$6::text::numeric,$7::text::numeric)
             RETURNING {FOOD_COLUMNS}"
        ))
        .bind(meal.id).bind(meal.user_id).bind(&meal.name)
        .bind(meal.calories_per_100g.to_string()).bind(meal.protein_per_100g.to_string())
        .bind(meal.carbs_per_100g.to_string()).bind(meal.fat_per_100g.to_string())
        .fetch_one(&self.pool).await?.into())
    }

    async fn update_meal(&self, meal: &Meal) -> Result<Option<Meal>, RepositoryError> {
        Ok(sqlx::query_as::<_, MealRow>(&format!(
            "UPDATE foods SET name=$1,calories_per_100g=$2::text::numeric,
               protein_per_100g=$3::text::numeric,carbs_per_100g=$4::text::numeric,fat_per_100g=$5::text::numeric
             WHERE id=$6 AND user_id=$7 AND archived_at IS NULL RETURNING {FOOD_COLUMNS}"
        ))
        .bind(&meal.name).bind(meal.calories_per_100g.to_string()).bind(meal.protein_per_100g.to_string())
        .bind(meal.carbs_per_100g.to_string()).bind(meal.fat_per_100g.to_string()).bind(meal.id).bind(meal.user_id)
        .fetch_optional(&self.pool).await?.map(Into::into))
    }

    async fn get_meal_by_id(
        &self,
        meal_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<Option<Meal>, RepositoryError> {
        Ok(sqlx::query_as::<_, MealRow>(&format!(
            "SELECT {FOOD_COLUMNS} FROM foods WHERE id=$1 AND user_id=$2 AND archived_at IS NULL"
        ))
        .bind(meal_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn get_all_meals(&self, user_id: &Uuid) -> Result<Vec<Meal>, RepositoryError> {
        Ok(sqlx::query_as::<_, MealRow>(&format!(
            "SELECT {FOOD_COLUMNS} FROM foods WHERE user_id=$1 AND archived_at IS NULL ORDER BY name,id"
        ))
        .bind(user_id).fetch_all(&self.pool).await?.into_iter().map(Into::into).collect())
    }

    async fn get_meals_paginated(
        &self,
        user_id: &Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<Meal>, i64), RepositoryError> {
        let offset = pagination_offset(page, per_page)?;
        let rows = sqlx::query_as::<_, MealRow>(&format!(
            "SELECT {FOOD_COLUMNS} FROM foods WHERE user_id=$1 AND archived_at IS NULL ORDER BY name,id LIMIT $2 OFFSET $3"
        ))
        .bind(user_id).bind(i64::from(per_page)).bind(offset).fetch_all(&self.pool).await?;
        let count = sqlx::query_scalar(
            "SELECT COUNT(*) FROM foods WHERE user_id=$1 AND archived_at IS NULL",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok((rows.into_iter().map(Into::into).collect(), count))
    }

    async fn delete_meal(&self, meal_id: &Uuid, user_id: &Uuid) -> Result<bool, RepositoryError> {
        Ok(sqlx::query("UPDATE foods SET archived_at=CURRENT_TIMESTAMP WHERE id=$1 AND user_id=$2 AND archived_at IS NULL")
            .bind(meal_id).bind(user_id).execute(&self.pool).await?.rows_affected() > 0)
    }
}

fn pagination_offset(page: u32, per_page: u32) -> Result<i64, RepositoryError> {
    if page == 0 || per_page == 0 || per_page > 100 {
        return Err(RepositoryError::Unexpected);
    }
    Ok(i64::from(page - 1) * i64::from(per_page))
}

#[async_trait]
impl ConsumptionRepository for SqlxNutritionRepository {
    async fn log_consumption(
        &self,
        consumption: &DailyConsumption,
    ) -> Result<DailyConsumption, RepositoryError> {
        let mut transaction = self.pool.begin().await?;
        let log_id = Uuid::new_v4();
        // The existing input supplies a day, not a time: explicitly mark the noon timestamp as estimated.
        sqlx::query(
            "INSERT INTO meal_logs (id,user_id,local_date,consumed_at,time_zone,time_is_estimated,meal_type)
             SELECT $1,id,$3,($3::date + TIME '12:00') AT TIME ZONE time_zone,time_zone,true,'other'
             FROM users WHERE id=$2"
        )
        .bind(log_id).bind(consumption.user_id).bind(consumption.date)
        .execute(&mut *transaction).await?;

        // Ownership and active state are checked in the same statement as the snapshot.
        // The old input's nutrient totals are deliberately not trusted or persisted.
        sqlx::query(
            "INSERT INTO meal_log_items
             (id,user_id,meal_log_id,food_id,position,quantity_grams,food_name_snapshot,
              calories_per_100g_snapshot,protein_per_100g_snapshot,carbs_per_100g_snapshot,fat_per_100g_snapshot)
             SELECT $1,user_id,$3,id,1,$5::text::numeric,name,
                    calories_per_100g,protein_per_100g,carbs_per_100g,fat_per_100g
             FROM foods WHERE user_id=$2 AND id=$4 AND archived_at IS NULL RETURNING id"
        )
        .bind(consumption.id).bind(consumption.user_id).bind(log_id).bind(consumption.meal_id)
        .bind(consumption.quantity_grams.to_string())
        .fetch_one(&mut *transaction).await?;
        let saved = sqlx::query_as::<_, ConsumptionRow>(&format!(
            "SELECT {CONSUMPTION_COLUMNS} FROM consumption_entries WHERE id=$1 AND user_id=$2"
        ))
        .bind(consumption.id)
        .bind(consumption.user_id)
        .fetch_one(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(saved.into())
    }

    async fn get_daily_consumption(
        &self,
        user_id: &Uuid,
        date: &NaiveDate,
    ) -> Result<Vec<DailyConsumption>, RepositoryError> {
        Ok(sqlx::query_as::<_, ConsumptionRow>(&format!(
            "SELECT {CONSUMPTION_COLUMNS} FROM consumption_entries WHERE user_id=$1 AND date=$2 ORDER BY id"
        ))
        .bind(user_id).bind(date).fetch_all(&self.pool).await?.into_iter().map(Into::into).collect())
    }

    async fn delete_consumption(
        &self,
        consumption_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let mut transaction = self.pool.begin().await?;
        let log: Option<Uuid> = sqlx::query_scalar(
            "DELETE FROM meal_log_items WHERE id=$1 AND user_id=$2 RETURNING meal_log_id",
        )
        .bind(consumption_id)
        .bind(user_id)
        .fetch_optional(&mut *transaction)
        .await?;
        let deleted = if let Some(log_id) = log {
            sqlx::query(
                "DELETE FROM meal_logs WHERE id=$1 AND user_id=$2
                 AND NOT EXISTS (SELECT 1 FROM meal_log_items WHERE meal_log_id=$1 AND user_id=$2)",
            )
            .bind(log_id)
            .bind(user_id)
            .execute(&mut *transaction)
            .await?;
            true
        } else {
            sqlx::query("DELETE FROM legacy.daily_consumption WHERE id=$1 AND user_id=$2")
                .bind(consumption_id)
                .bind(user_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected()
                > 0
        };
        transaction.commit().await?;
        Ok(deleted)
    }

    async fn get_consumptions_paginated(
        &self,
        user_id: &Uuid,
        page: u32,
        per_page: u32,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<(Vec<ConsumptionWithMeal>, i64), RepositoryError> {
        let offset = pagination_offset(page, per_page)?;
        let rows = sqlx::query_as::<_, ConsumptionWithMealRow>(&format!(
            "SELECT {ENTRY_COLUMNS} FROM consumption_entries
             WHERE user_id=$1 AND ($2::date IS NULL OR date >= $2) AND ($3::date IS NULL OR date <= $3)
             ORDER BY date DESC,id LIMIT $4 OFFSET $5"
        ))
        .bind(user_id).bind(start_date).bind(end_date).bind(i64::from(per_page)).bind(offset)
        .fetch_all(&self.pool).await?;
        let count = sqlx::query_scalar(
            "SELECT COUNT(*) FROM consumption_entries WHERE user_id=$1
             AND ($2::date IS NULL OR date >= $2) AND ($3::date IS NULL OR date <= $3)",
        )
        .bind(user_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&self.pool)
        .await?;
        Ok((rows.into_iter().map(Into::into).collect(), count))
    }

    async fn get_consumptions_by_date_range(
        &self,
        user_id: &Uuid,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
    ) -> Result<Vec<ConsumptionWithMeal>, RepositoryError> {
        Ok(sqlx::query_as::<_, ConsumptionWithMealRow>(&format!(
            "SELECT {ENTRY_COLUMNS} FROM consumption_entries
             WHERE user_id=$1 AND date >= $2 AND date <= $3 ORDER BY date DESC,id"
        ))
        .bind(user_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
    }

    async fn get_date_range_stats(
        &self,
        user_id: &Uuid,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
    ) -> Result<StatsSummary, RepositoryError> {
        Ok(sqlx::query_as::<_, StatsRow>(
            "SELECT COUNT(DISTINCT date) AS total_days,
             COALESCE(SUM(calories_consumed),0)::real AS total_calories,
             COALESCE(SUM(protein_consumed),0)::real AS total_protein,
             COALESCE(SUM(carbs_consumed),0)::real AS total_carbs,
             COALESCE(SUM(fat_consumed),0)::real AS total_fat
             FROM consumption_entries WHERE user_id=$1 AND date >= $2 AND date <= $3",
        )
        .bind(user_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&self.pool)
        .await?
        .into())
    }
}
