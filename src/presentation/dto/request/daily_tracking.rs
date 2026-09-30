use crate::domain::training::session::{ExerciseResult, PerformedSet, SessionUpdate};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StartSessionRequest {
    pub date: String,
    pub routine_id: Uuid,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionUpdateRequest {
    pub write_id: Uuid,
    pub revision: i32,
    pub complete: bool,
    pub exercises: Vec<ExerciseResultRequest>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExerciseResultRequest {
    pub id: Uuid,
    pub status: String,
    pub sets: Vec<SetRequest>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetRequest {
    pub reps: Option<i16>,
    pub load_kg: Option<f64>,
    pub duration_seconds: Option<i32>,
}
impl From<SessionUpdateRequest> for SessionUpdate {
    fn from(r: SessionUpdateRequest) -> Self {
        Self {
            write_id: r.write_id,
            revision: r.revision,
            complete: r.complete,
            exercises: r
                .exercises
                .into_iter()
                .map(|e| ExerciseResult {
                    id: e.id,
                    status: e.status,
                    sets: e
                        .sets
                        .into_iter()
                        .map(|s| PerformedSet {
                            reps: s.reps,
                            load_kg: s.load_kg,
                            duration_seconds: s.duration_seconds,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WaterRequest {
    pub id: Uuid,
    pub date: String,
    pub amount_ml: i32,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WaterGoalRequest {
    pub date: String,
    pub goal_ml: i32,
}
