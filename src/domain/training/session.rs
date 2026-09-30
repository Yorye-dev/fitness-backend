use crate::domain::errors::DomainError;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PerformedSet {
    pub reps: Option<i16>,
    pub load_kg: Option<f64>,
    pub duration_seconds: Option<i32>,
}
#[derive(Debug, Clone)]
pub struct SessionExercise {
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
    pub sets: Vec<PerformedSet>,
}
#[derive(Debug, Clone)]
pub struct WorkoutSession {
    pub id: Uuid,
    pub routine_id: Option<Uuid>,
    pub date: NaiveDate,
    pub name: String,
    pub status: String,
    pub revision: i32,
    pub exercises: Vec<SessionExercise>,
}
pub struct ExerciseResult {
    pub id: Uuid,
    pub status: String,
    pub sets: Vec<PerformedSet>,
}
pub struct SessionUpdate {
    pub write_id: Uuid,
    pub revision: i32,
    pub complete: bool,
    pub exercises: Vec<ExerciseResult>,
}
impl SessionUpdate {
    pub fn validate(&self) -> Result<(), DomainError> {
        let invalid = || {
            DomainError::Validation(
                "invalid exercise results; review statuses, sets and metrics".into(),
            )
        };
        if self.write_id.is_nil() || self.revision < 0 || !(1..=50).contains(&self.exercises.len())
        {
            return Err(invalid());
        }
        let mut seen = std::collections::HashSet::new();
        for e in &self.exercises {
            if e.id.is_nil()
                || !seen.insert(e.id)
                || !["pending", "completed", "skipped"].contains(&e.status.as_str())
                || !(1..=100).contains(&e.sets.len())
                || (self.complete && e.status == "pending")
            {
                return Err(invalid());
            }
            for s in &e.sets {
                if s.reps.is_some_and(|n| !(1..=1000).contains(&n))
                    || s.duration_seconds
                        .is_some_and(|n| !(1..=86400).contains(&n))
                    || s.load_kg.is_some_and(|n| {
                        !n.is_finite()
                            || !(0.0..100000.0).contains(&n)
                            || (n * 1000.0 - (n * 1000.0).round()).abs() >= 0.000001
                    })
                {
                    return Err(invalid());
                }
            }
        }
        if self.complete && !self.exercises.iter().any(|e| e.status == "completed") {
            return Err(invalid());
        }
        Ok(())
    }
    pub fn validate_against(&self, session: &WorkoutSession) -> Result<(), DomainError> {
        if self.exercises.len() != session.exercises.len() {
            return Err(DomainError::Validation(
                "include every session exercise".into(),
            ));
        }
        for input in &self.exercises {
            let original = session
                .exercises
                .iter()
                .find(|e| e.id == input.id)
                .ok_or_else(|| DomainError::Validation("unknown session exercise".into()))?;
            for s in &input.sets {
                let valid = if original.modality == "strength" {
                    s.duration_seconds.is_none()
                        && (input.status != "completed"
                            || (s.reps.is_some() && s.load_kg.is_some()))
                } else {
                    s.reps.is_none()
                        && s.load_kg.is_none()
                        && (input.status != "completed" || s.duration_seconds.is_some())
                };
                if !valid {
                    return Err(DomainError::Validation(
                        "completed exercises require actual metrics for every set".into(),
                    ));
                }
            }
        }
        Ok(())
    }
}
