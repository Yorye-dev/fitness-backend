use crate::domain::training::routine::{RoutineExercise, WeeklyDay, WorkoutRoutine};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct RoutineResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub exercises: Vec<ExerciseResponse>,
}
#[derive(Serialize)]
pub struct ExerciseResponse {
    pub name: String,
    pub modality: String,
    pub target_sets: i16,
    pub target_reps_min: Option<i16>,
    pub target_reps_max: Option<i16>,
    pub target_load_kg: Option<f64>,
    pub target_duration_seconds: Option<i32>,
    pub rest_seconds: i32,
    pub notes: String,
}
impl From<RoutineExercise> for ExerciseResponse {
    fn from(e: RoutineExercise) -> Self {
        Self {
            name: e.name,
            modality: e.modality,
            target_sets: e.target_sets,
            target_reps_min: e.target_reps_min,
            target_reps_max: e.target_reps_max,
            target_load_kg: e.target_load_kg,
            target_duration_seconds: e.target_duration_seconds,
            rest_seconds: e.rest_seconds,
            notes: e.notes,
        }
    }
}
impl From<WorkoutRoutine> for RoutineResponse {
    fn from(r: WorkoutRoutine) -> Self {
        Self {
            id: r.id,
            name: r.name,
            description: r.description,
            exercises: r.exercises.into_iter().map(Into::into).collect(),
        }
    }
}
#[derive(Serialize)]
pub struct WeeklyDayResponse {
    pub weekday: i16,
    pub routine_id: Option<Uuid>,
}
#[derive(Serialize)]
pub struct WeeklyScheduleResponse {
    pub days: Vec<WeeklyDayResponse>,
}
impl From<Vec<WeeklyDay>> for WeeklyScheduleResponse {
    fn from(days: Vec<WeeklyDay>) -> Self {
        Self {
            days: days
                .into_iter()
                .map(|d| WeeklyDayResponse {
                    weekday: d.weekday,
                    routine_id: d.routine_id,
                })
                .collect(),
        }
    }
}
#[derive(Serialize)]
pub struct DailyWorkoutResponse {
    pub date: String,
    pub weekday: u32,
    pub routine: Option<RoutineResponse>,
}
