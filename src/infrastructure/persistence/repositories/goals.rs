use crate::domain::{errors::RepositoryError, nutrition::goals::NutritionGoals};
use crate::infrastructure::persistence::models::nutrition_row::NutritionGoalsRow;
use sqlx::PgConnection;

// Preserve the current application/HTTP numeric contract at the adapter boundary.
// Persistence and generated consumption totals use NUMERIC throughout.
pub(super) const GOAL_COLUMNS: &str = "id,user_id,protein_target::real AS protein_goal,
    carbs_target::real AS carbs_goal,fat_target::real AS fats_goal,
    calories_target::real AS tdee,COALESCE(bmr_estimate,0)::real AS bmr";

/// Today's draft can be corrected; versions from earlier dates are never overwritten here.
pub(super) async fn save_current_goals(
    connection: &mut PgConnection,
    goals: &NutritionGoals,
    source: &str,
) -> Result<NutritionGoals, RepositoryError> {
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR UPDATE")
        .bind(goals.user_id)
        .fetch_one(&mut *connection)
        .await?;
    let query = format!(
        "INSERT INTO nutrition_goal_versions
         (user_id,effective_from,calories_target,protein_target,carbs_target,fat_target,bmr_estimate,tdee_estimate,source)
         SELECT id,(CURRENT_TIMESTAMP AT TIME ZONE time_zone)::date,
                $2::text::numeric,$3::text::numeric,$4::text::numeric,$5::text::numeric,
                CASE WHEN $6::text::numeric > 0 AND $6::text::numeric < 'Infinity'::numeric THEN $6::text::numeric END,
                CASE WHEN $7='profile' THEN $2::text::numeric END,$7
         FROM users WHERE id=$1
         ON CONFLICT(user_id,effective_from) DO UPDATE SET
           calories_target=EXCLUDED.calories_target,protein_target=EXCLUDED.protein_target,
           carbs_target=EXCLUDED.carbs_target,fat_target=EXCLUDED.fat_target,
           bmr_estimate=EXCLUDED.bmr_estimate,tdee_estimate=EXCLUDED.tdee_estimate,source=EXCLUDED.source
         RETURNING {GOAL_COLUMNS}"
    );
    Ok(sqlx::query_as::<_, NutritionGoalsRow>(&query)
        .bind(goals.user_id)
        .bind(goals.tdee.to_string())
        .bind(goals.protein_goal.to_string())
        .bind(goals.carbs_goal.to_string())
        .bind(goals.fats_goal.to_string())
        .bind(goals.bmr.to_string())
        .bind(source)
        .fetch_one(connection)
        .await?
        .into())
}
