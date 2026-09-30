use crate::domain::{
    hydration::DailyWater,
    training::session::{PerformedSet, WorkoutSession},
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct SetResponse {
    pub reps: Option<i16>,
    pub load_kg: Option<f64>,
    pub duration_seconds: Option<i32>,
}
impl From<PerformedSet> for SetResponse {
    fn from(s: PerformedSet) -> Self {
        Self {
            reps: s.reps,
            load_kg: s.load_kg,
            duration_seconds: s.duration_seconds,
        }
    }
}
#[derive(Serialize)]
pub struct SessionExerciseResponse {
    pub id: Uuid,
    pub exercise_id: Uuid,
    pub name: String,
    pub modality: String,
    pub status: String,
    pub target_sets: i16,
    pub target_reps_min: Option<i16>,
    pub target_reps_max: Option<i16>,
    pub target_load_kg: Option<f64>,
    pub target_duration_seconds: Option<i32>,
    pub notes: String,
    pub sets: Vec<SetResponse>,
}
#[derive(Serialize)]
pub struct SessionResponse {
    pub id: Uuid,
    pub routine_id: Option<Uuid>,
    pub date: String,
    pub name: String,
    pub status: String,
    pub revision: i32,
    pub exercises: Vec<SessionExerciseResponse>,
}
impl From<WorkoutSession> for SessionResponse {
    fn from(s: WorkoutSession) -> Self {
        Self {
            id: s.id,
            routine_id: s.routine_id,
            date: s.date.to_string(),
            name: s.name,
            status: s.status,
            revision: s.revision,
            exercises: s
                .exercises
                .into_iter()
                .map(|e| SessionExerciseResponse {
                    id: e.id,
                    exercise_id: e.exercise_id,
                    name: e.name,
                    modality: e.modality,
                    status: e.status,
                    target_sets: e.target_sets,
                    target_reps_min: e.target_reps_min,
                    target_reps_max: e.target_reps_max,
                    target_load_kg: e.target_load_kg,
                    target_duration_seconds: e.target_duration_seconds,
                    notes: e.notes,
                    sets: e.sets.into_iter().map(Into::into).collect(),
                })
                .collect(),
        }
    }
}
#[derive(Serialize)]
pub struct WaterEntryResponse {
    pub id: Uuid,
    pub amount_ml: i32,
}
#[derive(Serialize)]
pub struct DailyWaterResponse {
    pub date: String,
    pub goal_ml: i32,
    pub total_ml: i64,
    pub entries: Vec<WaterEntryResponse>,
}
impl From<DailyWater> for DailyWaterResponse {
    fn from(d: DailyWater) -> Self {
        Self {
            date: d.date.to_string(),
            goal_ml: d.goal_ml,
            total_ml: d.total_ml,
            entries: d
                .entries
                .into_iter()
                .map(|e| WaterEntryResponse {
                    id: e.id,
                    amount_ml: e.amount_ml,
                })
                .collect(),
        }
    }
}
