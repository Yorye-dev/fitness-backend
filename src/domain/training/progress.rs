use chrono::NaiveDate;
use uuid::Uuid;

pub struct SessionProgress {
    pub id: Uuid,
    pub date: NaiveDate,
    pub name: String,
    pub completed_exercises: i64,
    pub sets: i64,
    pub reps: i64,
    pub volume_kg: f64,
    pub duration_seconds: i64,
}
pub struct ProgressExercise {
    pub id: Uuid,
    pub name: String,
    pub modality: String,
}
pub struct ExerciseProgressPoint {
    pub session_id: Uuid,
    pub date: NaiveDate,
    pub sets: i64,
    pub reps: i64,
    pub max_load_kg: Option<f64>,
    pub volume_kg: f64,
    pub duration_seconds: i64,
}
pub struct WorkoutProgress {
    pub sessions: Vec<SessionProgress>,
    pub exercises: Vec<ProgressExercise>,
    pub selected_exercise_id: Option<Uuid>,
    pub points: Vec<ExerciseProgressPoint>,
}
