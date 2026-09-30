use super::goals::{GOAL_COLUMNS, save_current_goals};
use crate::domain::{
    errors::RepositoryError,
    nutrition::{
        consumption::{ConsumptionQuantity, DailyConsumption},
        goals::NutritionGoals,
        meal::{Meal, NutritionBasis},
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
const FOOD_COLUMNS: &str = "id,user_id,name,nutrition_basis='per_unit' AS per_unit,
    COALESCE(calories_per_100g,calories_per_unit)::real AS calories,
    COALESCE(protein_per_100g,protein_per_unit)::real AS protein,
    COALESCE(carbs_per_100g,carbs_per_unit)::real AS carbs,
    COALESCE(fat_per_100g,fat_per_unit)::real AS fat";
const CONSUMPTION_COLUMNS: &str = "id,user_id,date,meal_id,quantity_grams::double precision AS quantity_grams,
    portion_count::double precision AS portion_count,portion_grams::double precision AS portion_grams,
    calories_consumed::real AS calories_consumed,protein_consumed::real AS protein_consumed,
    carbs_consumed::real AS carbs_consumed,fat_consumed::real AS fat_consumed";
const ENTRY_COLUMNS: &str = "id,user_id,date,meal_id,quantity_grams::double precision AS quantity_grams,
    portion_count::double precision AS portion_count,portion_grams::double precision AS portion_grams,
    calories_consumed::real AS calories_consumed,protein_consumed::real AS protein_consumed,
    carbs_consumed::real AS carbs_consumed,fat_consumed::real AS fat_consumed,meal_name,
    COALESCE(calories_per_100g,calories_per_unit)::real AS meal_calories,
    COALESCE(protein_per_100g,protein_per_unit)::real AS meal_protein,
    COALESCE(carbs_per_100g,carbs_per_unit)::real AS meal_carbs,
    COALESCE(fat_per_100g,fat_per_unit)::real AS meal_fat";

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
        let weight = meal.nutrition_basis == NutritionBasis::Per100g;
        Ok(sqlx::query_as::<_, MealRow>(&format!(
            "INSERT INTO foods (id,user_id,name,calories_per_100g,protein_per_100g,carbs_per_100g,fat_per_100g,
             nutrition_basis,calories_per_unit,protein_per_unit,carbs_per_unit,fat_per_unit)
             VALUES ($1,$2,$3,$4::text::numeric,$5::text::numeric,$6::text::numeric,$7::text::numeric,
             $8,$9::text::numeric,$10::text::numeric,$11::text::numeric,$12::text::numeric)
             RETURNING {FOOD_COLUMNS}"
        ))
        .bind(meal.id).bind(meal.user_id).bind(&meal.name)
        .bind(weight.then(|| meal.calories.to_string())).bind(weight.then(|| meal.protein.to_string()))
        .bind(weight.then(|| meal.carbs.to_string())).bind(weight.then(|| meal.fat.to_string()))
        .bind(meal.nutrition_basis.as_str())
        .bind((!weight).then(|| meal.calories.to_string())).bind((!weight).then(|| meal.protein.to_string()))
        .bind((!weight).then(|| meal.carbs.to_string())).bind((!weight).then(|| meal.fat.to_string()))
        .fetch_one(&self.pool).await?.into())
    }

    async fn update_meal(&self, meal: &Meal) -> Result<Option<Meal>, RepositoryError> {
        let weight = meal.nutrition_basis == NutritionBasis::Per100g;
        Ok(sqlx::query_as::<_, MealRow>(&format!(
            "UPDATE foods SET name=$1,calories_per_100g=$2::text::numeric,
               protein_per_100g=$3::text::numeric,carbs_per_100g=$4::text::numeric,fat_per_100g=$5::text::numeric,
               nutrition_basis=$8,calories_per_unit=$9::text::numeric,protein_per_unit=$10::text::numeric,
               carbs_per_unit=$11::text::numeric,fat_per_unit=$12::text::numeric
             WHERE id=$6 AND user_id=$7 AND archived_at IS NULL RETURNING {FOOD_COLUMNS}"
        ))
        .bind(&meal.name).bind(weight.then(|| meal.calories.to_string())).bind(weight.then(|| meal.protein.to_string()))
        .bind(weight.then(|| meal.carbs.to_string())).bind(weight.then(|| meal.fat.to_string())).bind(meal.id).bind(meal.user_id)
        .bind(meal.nutrition_basis.as_str())
        .bind((!weight).then(|| meal.calories.to_string())).bind((!weight).then(|| meal.protein.to_string()))
        .bind((!weight).then(|| meal.carbs.to_string())).bind((!weight).then(|| meal.fat.to_string()))
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
        search: Option<&str>,
    ) -> Result<(Vec<Meal>, i64), RepositoryError> {
        let offset = pagination_offset(page, per_page)?;
        let rows = sqlx::query_as::<_, MealRow>(&format!(
            "SELECT {FOOD_COLUMNS} FROM foods WHERE user_id=$1 AND archived_at IS NULL
             AND ($2::text IS NULL OR strpos(lower(name), lower($2)) > 0)
             ORDER BY name,id LIMIT $3 OFFSET $4"
        ))
        .bind(user_id)
        .bind(search)
        .bind(i64::from(per_page))
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        let count = sqlx::query_scalar(
            "SELECT COUNT(*) FROM foods WHERE user_id=$1 AND archived_at IS NULL
             AND ($2::text IS NULL OR strpos(lower(name), lower($2)) > 0)",
        )
        .bind(user_id)
        .bind(search)
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
        // Serialize retries of one entry ID without blocking unrelated users/entries.
        sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
            .bind(consumption.id.to_string())
            .execute(&mut *transaction)
            .await?;
        let existing = sqlx::query_as::<_, ConsumptionRow>(&format!(
            "SELECT {CONSUMPTION_COLUMNS} FROM consumption_entries WHERE id=$1 AND user_id=$2"
        ))
        .bind(consumption.id)
        .bind(consumption.user_id)
        .fetch_optional(&mut *transaction)
        .await?;
        if let Some(existing) = existing {
            let matches: bool = sqlx::query_scalar(
                "SELECT EXISTS (SELECT 1 FROM consumption_entries WHERE id=$1 AND user_id=$2
                 AND date=$3 AND meal_id=$4 AND quantity_grams IS NOT DISTINCT FROM $5::text::numeric(10,3)
                 AND portion_count IS NOT DISTINCT FROM $6::text::numeric
                 AND portion_grams IS NOT DISTINCT FROM $7::text::numeric)",
            )
            .bind(consumption.id)
            .bind(consumption.user_id)
            .bind(consumption.date)
            .bind(consumption.meal_id)
            .bind(consumption.quantity_grams.map(|value| value.to_string()))
            .bind(consumption.portion_count.map(|value| value.to_string()))
            .bind(consumption.portion_grams.map(|value| value.to_string()))
            .fetch_one(&mut *transaction)
            .await?;
            if !matches {
                return Err(RepositoryError::Conflict);
            }
            transaction.commit().await?;
            return Ok(existing.into());
        }
        let per_unit: bool = sqlx::query_scalar(
            "SELECT nutrition_basis='per_unit' FROM foods WHERE id=$1 AND user_id=$2 AND archived_at IS NULL FOR SHARE"
        ).bind(consumption.meal_id).bind(consumption.user_id).fetch_one(&mut *transaction).await?;
        if per_unit != consumption.quantity_grams.is_none() {
            return Err(RepositoryError::Conflict);
        }
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
             (id,user_id,meal_log_id,food_id,position,quantity_grams,portion_count,portion_grams,food_name_snapshot,
              calories_per_100g_snapshot,protein_per_100g_snapshot,carbs_per_100g_snapshot,fat_per_100g_snapshot,
              nutrition_basis,calories_per_unit_snapshot,protein_per_unit_snapshot,carbs_per_unit_snapshot,fat_per_unit_snapshot)
             SELECT $1,user_id,$3,id,1,$5::text::numeric,$6::text::numeric,$7::text::numeric,name,
                    calories_per_100g,protein_per_100g,carbs_per_100g,fat_per_100g,
                    nutrition_basis,calories_per_unit,protein_per_unit,carbs_per_unit,fat_per_unit
             FROM foods WHERE user_id=$2 AND id=$4 AND archived_at IS NULL RETURNING id"
        )
        .bind(consumption.id).bind(consumption.user_id).bind(log_id).bind(consumption.meal_id)
        .bind(consumption.quantity_grams.map(|value| value.to_string()))
        .bind(consumption.portion_count.map(|value| value.to_string()))
        .bind(consumption.portion_grams.map(|value| value.to_string()))
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

    async fn update_consumption(
        &self,
        consumption_id: &Uuid,
        user_id: &Uuid,
        date: &NaiveDate,
        quantity: &ConsumptionQuantity,
    ) -> Result<Option<DailyConsumption>, RepositoryError> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
            .bind(consumption_id.to_string())
            .execute(&mut *transaction)
            .await?;
        // Lock the header so edits to siblings cannot move/delete it concurrently.
        let log: Option<(Uuid, NaiveDate, bool)> = sqlx::query_as(
            "SELECT l.id,l.local_date,i.nutrition_basis='per_unit' FROM meal_logs l
             JOIN meal_log_items i ON i.meal_log_id=l.id AND i.user_id=l.user_id
             WHERE i.id=$1 AND i.user_id=$2 FOR UPDATE OF l",
        )
        .bind(consumption_id)
        .bind(user_id)
        .fetch_optional(&mut *transaction)
        .await?;

        if let Some((old_log_id, old_date, per_unit)) = log {
            if per_unit != quantity.quantity_grams.is_none() {
                return Err(RepositoryError::Conflict);
            }
            let log_id = if old_date != *date {
                // Move just this item, preserving siblings and the original food snapshot.
                let id = Uuid::new_v4();
                sqlx::query(
                    "INSERT INTO meal_logs (id,user_id,local_date,consumed_at,time_zone,time_is_estimated,meal_type,notes)
                     SELECT $1,user_id,$4,($4::date + TIME '12:00') AT TIME ZONE time_zone,time_zone,true,meal_type,notes
                     FROM meal_logs WHERE id=$2 AND user_id=$3"
                ).bind(id).bind(old_log_id).bind(user_id).bind(date).execute(&mut *transaction).await?;
                id
            } else {
                old_log_id
            };
            sqlx::query(
                "UPDATE meal_log_items SET meal_log_id=$3,
                 position=CASE WHEN meal_log_id=$3 THEN position ELSE 1 END,
                 quantity_grams=$4::text::numeric,portion_count=$5::text::numeric,portion_grams=$6::text::numeric
                 WHERE id=$1 AND user_id=$2"
            ).bind(consumption_id).bind(user_id).bind(log_id)
                .bind(quantity.quantity_grams.map(|value| value.to_string()))
                .bind(quantity.portion_count.map(|value| value.to_string()))
                .bind(quantity.portion_grams.map(|value| value.to_string()))
                .execute(&mut *transaction).await?;
            if log_id != old_log_id {
                sqlx::query("DELETE FROM meal_logs WHERE id=$1 AND user_id=$2
                    AND NOT EXISTS (SELECT 1 FROM meal_log_items WHERE meal_log_id=$1 AND user_id=$2)")
                    .bind(old_log_id).bind(user_id).execute(&mut *transaction).await?;
            }
        } else {
            let old_quantity: Option<f64> = sqlx::query_scalar(
                "SELECT quantity_grams::double precision FROM legacy.daily_consumption
                 WHERE id=$1 AND user_id=$2 FOR UPDATE",
            )
            .bind(consumption_id)
            .bind(user_id)
            .fetch_optional(&mut *transaction)
            .await?;
            let Some(old_quantity) = old_quantity else {
                transaction.commit().await?;
                return Ok(None);
            };
            if !old_quantity.is_finite() || old_quantity <= 0.0 || quantity.quantity_grams.is_none()
            {
                return Err(RepositoryError::Conflict);
            }
            // Legacy has no snapshots: scale the recorded totals, never today's catalog values.
            // Keep same-quantity retries/date-only changes bit-for-bit unchanged.
            sqlx::query(
                "UPDATE legacy.daily_consumption SET date=$3,
                 calories_consumed=CASE WHEN quantity_grams=$4::text::real THEN calories_consumed ELSE
                    (calories_consumed::text::numeric*$4::text::numeric/quantity_grams::text::numeric)::real END,
                 protein_consumed=CASE WHEN quantity_grams=$4::text::real THEN protein_consumed ELSE
                    (protein_consumed::text::numeric*$4::text::numeric/quantity_grams::text::numeric)::real END,
                 carbs_consumed=CASE WHEN quantity_grams=$4::text::real THEN carbs_consumed ELSE
                    (carbs_consumed::text::numeric*$4::text::numeric/quantity_grams::text::numeric)::real END,
                 fat_consumed=CASE WHEN quantity_grams=$4::text::real THEN fat_consumed ELSE
                    (fat_consumed::text::numeric*$4::text::numeric/quantity_grams::text::numeric)::real END,
                 quantity_grams=$4::text::real,portion_count=$5::text::numeric,portion_grams=$6::text::numeric
                 WHERE id=$1 AND user_id=$2"
            ).bind(consumption_id).bind(user_id).bind(date).bind(quantity.quantity_grams.map(|value| value.to_string()))
                .bind(quantity.portion_count.map(|value| value.to_string()))
                .bind(quantity.portion_grams.map(|value| value.to_string()))
                .execute(&mut *transaction).await?;
        }
        let saved = sqlx::query_as::<_, ConsumptionRow>(&format!(
            "SELECT {CONSUMPTION_COLUMNS} FROM consumption_entries WHERE id=$1 AND user_id=$2"
        ))
        .bind(consumption_id)
        .bind(user_id)
        .fetch_one(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(Some(saved.into()))
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
        sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
            .bind(consumption_id.to_string())
            .execute(&mut *transaction)
            .await?;
        // Same header-before-item lock order as updates, including shared meal logs.
        sqlx::query(
            "SELECT l.id FROM meal_logs l JOIN meal_log_items i
            ON i.meal_log_id=l.id AND i.user_id=l.user_id
            WHERE i.id=$1 AND i.user_id=$2 FOR UPDATE OF l",
        )
        .bind(consumption_id)
        .bind(user_id)
        .execute(&mut *transaction)
        .await?;
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
