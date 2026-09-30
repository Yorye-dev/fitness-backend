pub use super::date::DateQuery as TrainingDateQuery;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProgressQuery {
    pub from: String,
    pub to: String,
    pub exercise_id: Option<uuid::Uuid>,
}
use crate::domain::training::routine::{RoutineExercise, WeeklyDay, WorkoutRoutine};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutineRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub exercises: Vec<ExerciseRequest>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExerciseRequest {
    pub name: String,
    pub modality: String,
    pub target_sets: i16,
    pub target_reps_min: Option<i16>,
    pub target_reps_max: Option<i16>,
    pub target_load_kg: Option<f64>,
    pub target_duration_seconds: Option<i32>,
    pub rest_seconds: i32,
    #[serde(default)]
    pub notes: String,
}
impl RoutineRequest {
    pub fn into_routine(self, id: Uuid) -> WorkoutRoutine {
        WorkoutRoutine {
            id,
            name: self.name,
            description: self.description,
            exercises: self
                .exercises
                .into_iter()
                .map(|e| RoutineExercise {
                    name: e.name,
                    modality: e.modality,
                    target_sets: e.target_sets,
                    target_reps_min: e.target_reps_min,
                    target_reps_max: e.target_reps_max,
                    target_load_kg: e.target_load_kg,
                    target_duration_seconds: e.target_duration_seconds,
                    rest_seconds: e.rest_seconds,
                    notes: e.notes,
                })
                .collect(),
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeeklyScheduleRequest {
    pub days: Vec<WeeklyDayRequest>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeeklyDayRequest {
    pub weekday: i16,
    pub routine_id: Option<Uuid>,
}
impl WeeklyScheduleRequest {
    pub fn into_days(self) -> Vec<WeeklyDay> {
        self.days
            .into_iter()
            .map(|d| WeeklyDay {
                weekday: d.weekday,
                routine_id: d.routine_id,
            })
            .collect()
    }
}
