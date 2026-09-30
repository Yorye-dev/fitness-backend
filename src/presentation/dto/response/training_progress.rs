use crate::domain::training::progress::{
    ExerciseProgressPoint, ProgressExercise, SessionProgress, WorkoutProgress,
};
use serde::Serialize;
use uuid::Uuid;
#[derive(Serialize)]
pub struct SessionProgressResponse {
    pub id: Uuid,
    pub date: String,
    pub name: String,
    pub completed_exercises: i64,
    pub sets: i64,
    pub reps: i64,
    pub volume_kg: f64,
    pub duration_seconds: i64,
}
impl From<SessionProgress> for SessionProgressResponse {
    fn from(value: SessionProgress) -> Self {
        Self {
            id: value.id,
            date: value.date.to_string(),
            name: value.name,
            completed_exercises: value.completed_exercises,
            sets: value.sets,
            reps: value.reps,
            volume_kg: value.volume_kg,
            duration_seconds: value.duration_seconds,
        }
    }
}
#[derive(Serialize)]
pub struct ProgressExerciseResponse {
    pub id: Uuid,
    pub name: String,
    pub modality: String,
}
impl From<ProgressExercise> for ProgressExerciseResponse {
    fn from(value: ProgressExercise) -> Self {
        Self {
            id: value.id,
            name: value.name,
            modality: value.modality,
        }
    }
}
#[derive(Serialize)]
pub struct ExerciseProgressPointResponse {
    pub session_id: Uuid,
    pub date: String,
    pub sets: i64,
    pub reps: i64,
    pub max_load_kg: Option<f64>,
    pub volume_kg: f64,
    pub duration_seconds: i64,
}
impl From<ExerciseProgressPoint> for ExerciseProgressPointResponse {
    fn from(value: ExerciseProgressPoint) -> Self {
        Self {
            session_id: value.session_id,
            date: value.date.to_string(),
            sets: value.sets,
            reps: value.reps,
            max_load_kg: value.max_load_kg,
            volume_kg: value.volume_kg,
            duration_seconds: value.duration_seconds,
        }
    }
}
#[derive(Serialize)]
pub struct WorkoutProgressResponse {
    pub sessions: Vec<SessionProgressResponse>,
    pub exercises: Vec<ProgressExerciseResponse>,
    pub selected_exercise_id: Option<Uuid>,
    pub points: Vec<ExerciseProgressPointResponse>,
}
impl From<WorkoutProgress> for WorkoutProgressResponse {
    fn from(value: WorkoutProgress) -> Self {
        Self {
            sessions: value.sessions.into_iter().map(Into::into).collect(),
            exercises: value.exercises.into_iter().map(Into::into).collect(),
            selected_exercise_id: value.selected_exercise_id,
            points: value.points.into_iter().map(Into::into).collect(),
        }
    }
}
