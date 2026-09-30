use crate::domain::{
    errors::RepositoryError,
    training::progress::{
        ExerciseProgressPoint, ProgressExercise, SessionProgress, WorkoutProgress,
    },
};
use chrono::NaiveDate;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(FromRow)]
struct SessionRow {
    id: Uuid,
    date: NaiveDate,
    name: String,
    completed_exercises: i64,
    sets: i64,
    reps: i64,
    volume_kg: f64,
    duration_seconds: i64,
}
#[derive(FromRow)]
struct ExerciseRow {
    id: Uuid,
    name: String,
    modality: String,
}
#[derive(FromRow)]
struct PointRow {
    session_id: Uuid,
    date: NaiveDate,
    sets: i64,
    reps: i64,
    max_load_kg: Option<f64>,
    volume_kg: f64,
    duration_seconds: i64,
}

pub(super) async fn load(
    pool: &PgPool,
    user: Uuid,
    from: NaiveDate,
    to: NaiveDate,
    exercise: Option<Uuid>,
) -> Result<WorkoutProgress, RepositoryError> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
        .execute(&mut *tx)
        .await?;
    let sessions=sqlx::query_as::<_,SessionRow>(
        "SELECT s.id,s.local_date AS date,s.name_snapshot AS name,
         COUNT(DISTINCT e.id) FILTER (WHERE w.id IS NOT NULL) AS completed_exercises,COUNT(w.id) AS sets,
         COALESCE(SUM(w.reps) FILTER (WHERE e.modality_snapshot='strength'),0)::bigint AS reps,
         COALESCE(SUM(w.load_kg*w.reps) FILTER (WHERE e.modality_snapshot='strength'),0)::double precision AS volume_kg,
         COALESCE(SUM(w.duration_seconds) FILTER (WHERE e.modality_snapshot IN ('cardio','mobility')),0)::bigint AS duration_seconds
         FROM workout_sessions s
         LEFT JOIN session_exercises e ON e.user_id=s.user_id AND e.session_id=s.id AND e.status='completed'
         LEFT JOIN workout_sets w ON w.user_id=e.user_id AND w.session_exercise_id=e.id AND w.status='completed'
         WHERE s.user_id=$1 AND s.is_daily AND s.status='completed' AND s.local_date BETWEEN $2 AND $3
         GROUP BY s.id ORDER BY s.local_date DESC,s.id DESC")
        .bind(user).bind(from).bind(to).fetch_all(&mut *tx).await?;
    // Names come from the most recent snapshot in this range, including archived catalog entries.
    let exercises=sqlx::query_as::<_,ExerciseRow>(
        "SELECT id,name,modality FROM (
         SELECT DISTINCT ON(e.exercise_id) e.exercise_id AS id,e.exercise_name_snapshot AS name,e.modality_snapshot AS modality
         FROM session_exercises e JOIN workout_sessions s ON s.user_id=e.user_id AND s.id=e.session_id
         WHERE s.user_id=$1 AND s.is_daily AND s.status='completed' AND s.local_date BETWEEN $2 AND $3 AND e.status='completed'
         AND EXISTS(SELECT 1 FROM workout_sets w WHERE w.user_id=e.user_id AND w.session_exercise_id=e.id AND w.status='completed')
         ORDER BY e.exercise_id,s.local_date DESC,s.id DESC,e.position DESC
         ) available ORDER BY name,id")
        .bind(user).bind(from).bind(to).fetch_all(&mut *tx).await?;
    let selected = exercise
        .filter(|id| exercises.iter().any(|e| e.id == *id))
        .or_else(|| exercises.first().map(|e| e.id));
    let points = if let Some(id) = selected {
        sqlx::query_as::<_,PointRow>(
            "SELECT s.id AS session_id,s.local_date AS date,COUNT(w.id) AS sets,
             COALESCE(SUM(w.reps) FILTER (WHERE e.modality_snapshot='strength'),0)::bigint AS reps,
             (MAX(w.load_kg) FILTER (WHERE e.modality_snapshot='strength'))::double precision AS max_load_kg,
             COALESCE(SUM(w.load_kg*w.reps) FILTER (WHERE e.modality_snapshot='strength'),0)::double precision AS volume_kg,
             COALESCE(SUM(w.duration_seconds) FILTER (WHERE e.modality_snapshot IN ('cardio','mobility')),0)::bigint AS duration_seconds
             FROM workout_sessions s
             JOIN session_exercises e ON e.user_id=s.user_id AND e.session_id=s.id AND e.status='completed'
             JOIN workout_sets w ON w.user_id=e.user_id AND w.session_exercise_id=e.id AND w.status='completed'
             WHERE s.user_id=$1 AND s.is_daily AND s.status='completed' AND s.local_date BETWEEN $2 AND $3 AND e.exercise_id=$4
             GROUP BY s.id ORDER BY s.local_date,s.id")
            .bind(user).bind(from).bind(to).bind(id).fetch_all(&mut *tx).await?
    } else {
        Vec::new()
    };
    tx.commit().await?;
    Ok(WorkoutProgress {
        selected_exercise_id: selected,
        sessions: sessions
            .into_iter()
            .map(|s| SessionProgress {
                id: s.id,
                date: s.date,
                name: s.name,
                completed_exercises: s.completed_exercises,
                sets: s.sets,
                reps: s.reps,
                volume_kg: s.volume_kg,
                duration_seconds: s.duration_seconds,
            })
            .collect(),
        exercises: exercises
            .into_iter()
            .map(|e| ProgressExercise {
                id: e.id,
                name: e.name,
                modality: e.modality,
            })
            .collect(),
        points: points
            .into_iter()
            .map(|p| ExerciseProgressPoint {
                session_id: p.session_id,
                date: p.date,
                sets: p.sets,
                reps: p.reps,
                max_load_kg: p.max_load_kg,
                volume_kg: p.volume_kg,
                duration_seconds: p.duration_seconds,
            })
            .collect(),
    })
}
