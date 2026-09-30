use crate::domain::{
    errors::RepositoryError,
    training::{
        repository::TrainingRepository,
        routine::{RoutineExercise, WeeklyDay, WorkoutRoutine},
    },
};
use async_trait::async_trait;
use sqlx::{FromRow, PgConnection, PgPool};
use uuid::Uuid;

#[derive(Clone)]
pub struct SqlxTrainingRepository {
    pool: PgPool,
}
impl SqlxTrainingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
#[derive(FromRow)]
struct RoutineRow {
    id: Uuid,
    name: String,
    description: String,
}
#[derive(FromRow)]
struct ExerciseRow {
    routine_id: Uuid,
    name: String,
    modality: String,
    target_sets: i16,
    target_reps_min: Option<i16>,
    target_reps_max: Option<i16>,
    target_load_kg: Option<f64>,
    target_duration_seconds: Option<i32>,
    rest_seconds: i32,
    notes: String,
}
#[derive(FromRow)]
struct DayRow {
    weekday: i16,
    routine_id: Option<Uuid>,
}

async fn load_routines(
    connection: &mut PgConnection,
    user_id: Uuid,
    routine_id: Option<Uuid>,
) -> Result<Vec<WorkoutRoutine>, RepositoryError> {
    let rows = sqlx::query_as::<_, RoutineRow>(
        "SELECT id,name,COALESCE(description,'') AS description FROM workout_routines
         WHERE user_id=$1 AND archived_at IS NULL AND ($2::uuid IS NULL OR id=$2) ORDER BY name,id",
    )
    .bind(user_id)
    .bind(routine_id)
    .fetch_all(&mut *connection)
    .await?;
    let exercises = sqlx::query_as::<_, ExerciseRow>(
        "SELECT re.routine_id,e.name,e.modality,re.target_sets,re.target_reps_min,re.target_reps_max,
         re.target_load_kg::double precision AS target_load_kg,re.target_duration_seconds,re.rest_seconds,
         COALESCE(re.notes,'') AS notes FROM routine_exercises re
         JOIN exercises e ON e.id=re.exercise_id AND e.user_id=re.user_id
         JOIN workout_routines r ON r.id=re.routine_id AND r.user_id=re.user_id
         WHERE re.user_id=$1 AND r.archived_at IS NULL AND ($2::uuid IS NULL OR re.routine_id=$2)
         ORDER BY re.routine_id,re.position"
    ).bind(user_id).bind(routine_id).fetch_all(&mut *connection).await?;
    let mut grouped = std::collections::HashMap::<Uuid, Vec<RoutineExercise>>::new();
    for e in exercises {
        grouped
            .entry(e.routine_id)
            .or_default()
            .push(RoutineExercise {
                name: e.name,
                modality: e.modality,
                target_sets: e.target_sets,
                target_reps_min: e.target_reps_min,
                target_reps_max: e.target_reps_max,
                target_load_kg: e.target_load_kg,
                target_duration_seconds: e.target_duration_seconds,
                rest_seconds: e.rest_seconds,
                notes: e.notes,
            });
    }
    Ok(rows
        .into_iter()
        .map(|r| WorkoutRoutine {
            id: r.id,
            name: r.name,
            description: r.description,
            exercises: grouped.remove(&r.id).unwrap_or_default(),
        })
        .collect())
}
async fn load_week(
    connection: &mut PgConnection,
    user_id: Uuid,
) -> Result<Vec<WeeklyDay>, RepositoryError> {
    let rows = sqlx::query_as::<_, DayRow>(
        "SELECT day::smallint AS weekday,s.routine_id FROM generate_series(1,7) day
         LEFT JOIN weekly_workout_schedule s ON s.weekday=day AND s.user_id=$1 ORDER BY day",
    )
    .bind(user_id)
    .fetch_all(connection)
    .await?;
    Ok(rows
        .into_iter()
        .map(|d| WeeklyDay {
            weekday: d.weekday,
            routine_id: d.routine_id,
        })
        .collect())
}
async fn lock_owner(connection: &mut PgConnection, user_id: Uuid) -> Result<(), RepositoryError> {
    // Uniform lock order for all training writes, including archive versus assignment.
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR UPDATE")
        .bind(user_id)
        .fetch_one(connection)
        .await?;
    Ok(())
}

#[async_trait]
impl TrainingRepository for SqlxTrainingRepository {
    async fn list_routines(&self, user_id: Uuid) -> Result<Vec<WorkoutRoutine>, RepositoryError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
            .execute(&mut *tx)
            .await?;
        let routines = load_routines(&mut tx, user_id, None).await?;
        tx.commit().await?;
        Ok(routines)
    }
    async fn save_routine(
        &self,
        user_id: Uuid,
        routine: &WorkoutRoutine,
    ) -> Result<WorkoutRoutine, RepositoryError> {
        let mut tx = self.pool.begin().await?;
        lock_owner(&mut tx, user_id).await?;
        let exists: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM workout_routines WHERE id=$1 AND user_id=$2 AND archived_at IS NULL)")
            .bind(routine.id).bind(user_id).fetch_one(&mut *tx).await?;
        if !exists {
            let count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM workout_routines WHERE user_id=$1 AND archived_at IS NULL",
            )
            .bind(user_id)
            .fetch_one(&mut *tx)
            .await?;
            if count >= 100 {
                return Err(RepositoryError::Conflict);
            }
        }
        let saved = sqlx::query(
            "INSERT INTO workout_routines (id,user_id,name,description) VALUES ($1,$2,$3,$4)
             ON CONFLICT (id) DO UPDATE SET name=EXCLUDED.name,description=EXCLUDED.description
             WHERE workout_routines.user_id=$2 AND workout_routines.archived_at IS NULL RETURNING id"
        ).bind(routine.id).bind(user_id).bind(&routine.name).bind(&routine.description)
            .fetch_optional(&mut *tx).await?;
        if saved.is_none() {
            return Err(RepositoryError::Conflict);
        }
        sqlx::query("DELETE FROM routine_exercises WHERE routine_id=$1 AND user_id=$2")
            .bind(routine.id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        for (index, exercise) in routine.exercises.iter().enumerate() {
            // Reuse the user's exercise catalog; renaming a row never changes another routine.
            let existing: Option<Uuid> = sqlx::query_scalar(
                "SELECT id FROM exercises WHERE user_id=$1 AND name=$2 AND modality=$3 AND archived_at IS NULL ORDER BY id LIMIT 1"
            ).bind(user_id).bind(&exercise.name).bind(&exercise.modality).fetch_optional(&mut *tx).await?;
            let exercise_id = match existing {
                Some(id) => id,
                None => sqlx::query_scalar(
                    "INSERT INTO exercises (user_id,name,modality) VALUES ($1,$2,$3) RETURNING id",
                )
                .bind(user_id)
                .bind(&exercise.name)
                .bind(&exercise.modality)
                .fetch_one(&mut *tx)
                .await?,
            };
            sqlx::query(
                "INSERT INTO routine_exercises
                 (user_id,routine_id,exercise_id,position,target_sets,target_reps_min,target_reps_max,
                  target_load_kg,target_duration_seconds,rest_seconds,notes)
                 VALUES ($1,$2,$3,$4,$5,$6,$7,$8::text::numeric,$9,$10,$11)"
            ).bind(user_id).bind(routine.id).bind(exercise_id).bind((index+1) as i32)
                .bind(exercise.target_sets).bind(exercise.target_reps_min).bind(exercise.target_reps_max)
                .bind(exercise.target_load_kg.map(|v| v.to_string())).bind(exercise.target_duration_seconds)
                .bind(exercise.rest_seconds).bind(&exercise.notes).execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(routine.clone())
    }
    async fn archive_routine(&self, user_id: Uuid, id: Uuid) -> Result<bool, RepositoryError> {
        let mut tx = self.pool.begin().await?;
        lock_owner(&mut tx, user_id).await?;
        let archived = sqlx::query(
            "UPDATE workout_routines SET archived_at=CURRENT_TIMESTAMP WHERE id=$1 AND user_id=$2 AND archived_at IS NULL"
        ).bind(id).bind(user_id).execute(&mut *tx).await?.rows_affected() > 0;
        if archived {
            sqlx::query("UPDATE weekly_workout_schedule SET routine_id=NULL WHERE routine_id=$1 AND user_id=$2")
                .bind(id).bind(user_id).execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(archived)
    }
    async fn get_week(&self, user_id: Uuid) -> Result<Vec<WeeklyDay>, RepositoryError> {
        load_week(&mut *self.pool.acquire().await?, user_id).await
    }
    async fn save_week(
        &self,
        user_id: Uuid,
        days: &[WeeklyDay],
    ) -> Result<Vec<WeeklyDay>, RepositoryError> {
        let mut tx = self.pool.begin().await?;
        lock_owner(&mut tx, user_id).await?;
        for day in days {
            if let Some(id) = day.routine_id {
                let active: bool = sqlx::query_scalar(
                    "SELECT EXISTS (SELECT 1 FROM workout_routines WHERE id=$1 AND user_id=$2 AND archived_at IS NULL)"
                ).bind(id).bind(user_id).fetch_one(&mut *tx).await?;
                if !active {
                    return Err(RepositoryError::NotFound);
                }
            }
            sqlx::query(
                "INSERT INTO weekly_workout_schedule (user_id,weekday,routine_id) VALUES ($1,$2,$3)
                 ON CONFLICT (user_id,weekday) DO UPDATE SET routine_id=EXCLUDED.routine_id",
            )
            .bind(user_id)
            .bind(day.weekday)
            .bind(day.routine_id)
            .execute(&mut *tx)
            .await?;
        }
        let saved = load_week(&mut tx, user_id).await?;
        tx.commit().await?;
        Ok(saved)
    }
    async fn get_daily_routine(
        &self,
        user_id: Uuid,
        weekday: i16,
    ) -> Result<Option<WorkoutRoutine>, RepositoryError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
            .execute(&mut *tx)
            .await?;
        let id: Option<Uuid> = sqlx::query_scalar(
            "SELECT routine_id FROM weekly_workout_schedule WHERE user_id=$1 AND weekday=$2",
        )
        .bind(user_id)
        .bind(weekday)
        .fetch_optional(&mut *tx)
        .await?
        .flatten();
        let routine = if let Some(id) = id {
            load_routines(&mut tx, user_id, Some(id))
                .await?
                .into_iter()
                .find(|r| r.id == id)
        } else {
            None
        };
        tx.commit().await?;
        Ok(routine)
    }
}
