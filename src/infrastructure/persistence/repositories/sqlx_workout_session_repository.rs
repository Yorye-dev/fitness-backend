use crate::domain::{
    errors::RepositoryError,
    training::{
        session::{PerformedSet, SessionExercise, SessionUpdate, WorkoutSession},
        session_repository::{SessionWriteError, WorkoutSessionRepository},
    },
};
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::{FromRow, PgConnection, PgPool};
use uuid::Uuid;

pub struct SqlxWorkoutSessionRepository(PgPool);
impl SqlxWorkoutSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}
impl From<sqlx::Error> for SessionWriteError {
    fn from(error: sqlx::Error) -> Self {
        Self::Repository(RepositoryError::from(error))
    }
}
#[derive(FromRow)]
struct Header {
    id: Uuid,
    routine_id: Option<Uuid>,
    local_date: NaiveDate,
    name_snapshot: String,
    status: String,
    revision: i32,
}
#[derive(FromRow)]
struct Exercise {
    id: Uuid,
    exercise_id: Uuid,
    exercise_name_snapshot: String,
    modality_snapshot: String,
    status: String,
    target_sets: i16,
    target_reps_min: Option<i16>,
    target_reps_max: Option<i16>,
    target_load_kg: Option<f64>,
    target_duration_seconds: Option<i32>,
    notes: String,
}
#[derive(FromRow)]
struct SetRow {
    session_exercise_id: Uuid,
    reps: Option<i16>,
    load_kg: Option<f64>,
    duration_seconds: Option<i32>,
}

async fn load(
    c: &mut PgConnection,
    user: Uuid,
    id: Uuid,
) -> Result<WorkoutSession, RepositoryError> {
    let h=sqlx::query_as::<_,Header>("SELECT id,routine_id,local_date,name_snapshot,status,revision FROM workout_sessions WHERE user_id=$1 AND id=$2 AND is_daily")
        .bind(user).bind(id).fetch_optional(&mut *c).await?.ok_or(RepositoryError::NotFound)?;
    // An untyped integer fallback promotes SMALLINT to INT4, which SQLx cannot decode as i16.
    let rows=sqlx::query_as::<_,Exercise>("SELECT id,exercise_id,exercise_name_snapshot,modality_snapshot,status,COALESCE(target_sets,1::smallint) AS target_sets,target_reps_min,target_reps_max,target_load_kg::double precision AS target_load_kg,target_duration_seconds,COALESCE(notes,'') AS notes FROM session_exercises WHERE user_id=$1 AND session_id=$2 ORDER BY position")
        .bind(user).bind(id).fetch_all(&mut *c).await?;
    let sets=sqlx::query_as::<_,SetRow>("SELECT ws.session_exercise_id,ws.reps,ws.load_kg::double precision AS load_kg,ws.duration_seconds FROM workout_sets ws JOIN session_exercises e ON e.user_id=ws.user_id AND e.id=ws.session_exercise_id WHERE e.user_id=$1 AND e.session_id=$2 ORDER BY e.position,ws.set_number")
        .bind(user).bind(id).fetch_all(&mut *c).await?;
    let mut grouped = std::collections::HashMap::<Uuid, Vec<PerformedSet>>::new();
    for s in sets {
        grouped
            .entry(s.session_exercise_id)
            .or_default()
            .push(PerformedSet {
                reps: s.reps,
                load_kg: s.load_kg,
                duration_seconds: s.duration_seconds,
            });
    }
    Ok(WorkoutSession {
        id: h.id,
        routine_id: h.routine_id,
        date: h.local_date,
        name: h.name_snapshot,
        status: h.status,
        revision: h.revision,
        exercises: rows
            .into_iter()
            .map(|e| SessionExercise {
                id: e.id,
                exercise_id: e.exercise_id,
                name: e.exercise_name_snapshot,
                modality: e.modality_snapshot,
                status: e.status,
                target_sets: e.target_sets,
                target_reps_min: e.target_reps_min,
                target_reps_max: e.target_reps_max,
                target_load_kg: e.target_load_kg,
                target_duration_seconds: e.target_duration_seconds,
                notes: e.notes,
                sets: grouped.remove(&e.id).unwrap_or_default(),
            })
            .collect(),
    })
}
async fn lock_owner(c: &mut PgConnection, user: Uuid) -> Result<(), RepositoryError> {
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR UPDATE")
        .bind(user)
        .fetch_one(c)
        .await?;
    Ok(())
}
#[async_trait]
impl WorkoutSessionRepository for SqlxWorkoutSessionRepository {
    async fn progress(
        &self,
        user: Uuid,
        from: NaiveDate,
        to: NaiveDate,
        exercise: Option<Uuid>,
    ) -> Result<crate::domain::training::progress::WorkoutProgress, RepositoryError> {
        super::training_progress::load(&self.0, user, from, to, exercise).await
    }
    async fn daily(
        &self,
        user: Uuid,
        date: NaiveDate,
    ) -> Result<Option<WorkoutSession>, RepositoryError> {
        let mut tx = self.0.begin().await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
            .execute(&mut *tx)
            .await?;
        let id: Option<Uuid> = sqlx::query_scalar(
            "SELECT id FROM workout_sessions WHERE user_id=$1 AND local_date=$2 AND is_daily",
        )
        .bind(user)
        .bind(date)
        .fetch_optional(&mut *tx)
        .await?;
        let session = match id {
            Some(id) => Some(load(&mut tx, user, id).await?),
            None => None,
        };
        tx.commit().await?;
        Ok(session)
    }
    async fn start(
        &self,
        user: Uuid,
        date: NaiveDate,
        routine: Uuid,
    ) -> Result<WorkoutSession, RepositoryError> {
        let mut tx = self.0.begin().await?;
        lock_owner(&mut tx, user).await?;
        // One durable daily slot also makes retries safe after a lost response.
        let existing: Option<Uuid> = sqlx::query_scalar(
            "SELECT id FROM workout_sessions WHERE user_id=$1 AND local_date=$2 AND is_daily",
        )
        .bind(user)
        .bind(date)
        .fetch_optional(&mut *tx)
        .await?;
        if let Some(id) = existing {
            let saved = load(&mut tx, user, id).await?;
            tx.commit().await?;
            return Ok(saved);
        }
        let id:Option<Uuid>=sqlx::query_scalar(
            "INSERT INTO workout_sessions(user_id,routine_id,name_snapshot,local_date,time_zone,started_at,is_daily,time_is_estimated)
             SELECT u.id,r.id,r.name,$3,u.time_zone,
             CASE WHEN $3=(CURRENT_TIMESTAMP AT TIME ZONE u.time_zone)::date THEN CURRENT_TIMESTAMP ELSE ($3::date+TIME '12:00') AT TIME ZONE u.time_zone END,
             true,$3<>(CURRENT_TIMESTAMP AT TIME ZONE u.time_zone)::date
             FROM users u JOIN workout_routines r ON r.user_id=u.id WHERE u.id=$1 AND r.id=$2 AND r.archived_at IS NULL
             AND EXISTS(SELECT 1 FROM routine_exercises re WHERE re.user_id=u.id AND re.routine_id=r.id) RETURNING id")
            .bind(user).bind(routine).bind(date).fetch_optional(&mut *tx).await?;
        let id = id.ok_or(RepositoryError::NotFound)?;
        sqlx::query("INSERT INTO session_exercises(user_id,session_id,exercise_id,position,exercise_name_snapshot,modality_snapshot,target_sets,target_reps_min,target_reps_max,target_load_kg,target_duration_seconds,target_distance_m,rest_seconds,notes)
            SELECT re.user_id,$3,re.exercise_id,re.position,e.name,e.modality,re.target_sets,re.target_reps_min,re.target_reps_max,re.target_load_kg,re.target_duration_seconds,re.target_distance_m,re.rest_seconds,re.notes
            FROM routine_exercises re JOIN exercises e ON e.user_id=re.user_id AND e.id=re.exercise_id WHERE re.user_id=$1 AND re.routine_id=$2")
            .bind(user).bind(routine).bind(id).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO workout_sets(user_id,session_exercise_id,set_number,reps,load_kg,duration_seconds)
            SELECT user_id,id,n::smallint,target_reps_min,target_load_kg,target_duration_seconds FROM session_exercises CROSS JOIN LATERAL generate_series(1,target_sets) n WHERE user_id=$1 AND session_id=$2")
            .bind(user).bind(id).execute(&mut *tx).await?;
        let saved = load(&mut tx, user, id).await?;
        tx.commit().await?;
        Ok(saved)
    }
    async fn save(
        &self,
        user: Uuid,
        id: Uuid,
        update: &SessionUpdate,
    ) -> Result<WorkoutSession, SessionWriteError> {
        let mut tx = self.0.begin().await?;
        lock_owner(&mut tx, user).await?;
        let current = load(&mut tx, user, id).await?;
        let last_write: Option<Uuid> = sqlx::query_scalar(
            "SELECT last_write_id FROM workout_sessions WHERE id=$1 AND user_id=$2",
        )
        .bind(id)
        .bind(user)
        .fetch_one(&mut *tx)
        .await?;
        if last_write == Some(update.write_id) {
            tx.commit().await?;
            return Ok(current);
        }
        if update.revision != current.revision
            || (current.status == "completed" && !update.complete)
        {
            return Err(RepositoryError::Conflict.into());
        }
        update.validate_against(&current)?;
        for e in &update.exercises {
            sqlx::query("UPDATE session_exercises SET status=$3 WHERE user_id=$1 AND id=$2")
                .bind(user)
                .bind(e.id)
                .bind(&e.status)
                .execute(&mut *tx)
                .await?;
            sqlx::query("DELETE FROM workout_sets WHERE user_id=$1 AND session_exercise_id=$2")
                .bind(user)
                .bind(e.id)
                .execute(&mut *tx)
                .await?;
            let status = match e.status.as_str() {
                "completed" => "completed",
                "skipped" => "skipped",
                _ => "pending",
            };
            for (index, s) in e.sets.iter().enumerate() {
                // Skipped metrics are NULL, so they cannot be mistaken for performed work in statistics.
                let skip = status == "skipped";
                sqlx::query("INSERT INTO workout_sets(user_id,session_exercise_id,set_number,status,reps,load_kg,duration_seconds,completed_at)
                    VALUES($1,$2,$3,$4,$5,$6::text::numeric,$7,CASE WHEN $4='completed' THEN CURRENT_TIMESTAMP ELSE NULL END)")
                    .bind(user).bind(e.id).bind((index+1) as i16).bind(status)
                    .bind(if skip {None}else{s.reps}).bind(if skip {None}else{s.load_kg.map(|n|n.to_string())})
                    .bind(if skip {None}else{s.duration_seconds}).execute(&mut *tx).await?;
            }
        }
        sqlx::query("UPDATE workout_sessions SET status=$3,finished_at=CASE WHEN $3='completed' THEN COALESCE(finished_at,GREATEST(started_at,CURRENT_TIMESTAMP)) ELSE NULL END,revision=revision+1,last_write_id=$4 WHERE user_id=$1 AND id=$2")
            .bind(user).bind(id).bind(if update.complete {"completed"} else {"in_progress"}).bind(update.write_id).execute(&mut *tx).await?;
        let saved = load(&mut tx, user, id).await?;
        tx.commit().await?;
        Ok(saved)
    }
}
