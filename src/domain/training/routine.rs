use crate::domain::errors::DomainError;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RoutineExercise {
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

#[derive(Debug, Clone)]
pub struct WorkoutRoutine {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub exercises: Vec<RoutineExercise>,
}

impl WorkoutRoutine {
    pub fn validate(mut self) -> Result<Self, DomainError> {
        self.name = self.name.trim().to_owned();
        self.description = self.description.trim().to_owned();
        if self.id.is_nil()
            || !(1..=200).contains(&self.name.chars().count())
            || self.description.chars().count() > 2000
            || !(1..=50).contains(&self.exercises.len())
        {
            return Err(DomainError::Validation(
                "invalid routine; provide a name and between 1 and 50 exercises".into(),
            ));
        }
        for exercise in &mut self.exercises {
            exercise.name = exercise.name.trim().to_owned();
            exercise.notes = exercise.notes.trim().to_owned();
            if !(1..=200).contains(&exercise.name.chars().count())
                || exercise.notes.chars().count() > 1000
                || !(1..=100).contains(&exercise.target_sets)
                || !(0..=3600).contains(&exercise.rest_seconds)
            {
                return Err(DomainError::Validation(
                    "invalid exercise name, sets, rest or notes".into(),
                ));
            }
            if exercise.target_load_kg.is_some_and(|weight| {
                !weight.is_finite()
                    || !(0.0..100_000.0).contains(&weight)
                    || (weight * 1000.0 - (weight * 1000.0).round()).abs() >= 0.000001
            }) {
                return Err(DomainError::Validation(
                    "invalid target load (up to 3 decimals)".into(),
                ));
            }
            match exercise.modality.as_str() {
                "strength" => {
                    if !matches!((exercise.target_reps_min, exercise.target_reps_max),
                        (Some(min), Some(max)) if (1..=1000).contains(&min) && (min..=1000).contains(&max))
                        || exercise.target_duration_seconds.is_some()
                    {
                        return Err(DomainError::Validation(
                            "strength exercises require a valid repetition range".into(),
                        ));
                    }
                }
                "cardio" | "mobility" => {
                    if !exercise
                        .target_duration_seconds
                        .is_some_and(|seconds| (1..=86400).contains(&seconds))
                        || exercise.target_reps_min.is_some()
                        || exercise.target_reps_max.is_some()
                        || exercise.target_load_kg.is_some()
                    {
                        return Err(DomainError::Validation(
                            "cardio and mobility require a duration, without reps or load".into(),
                        ));
                    }
                }
                _ => return Err(DomainError::Validation("invalid exercise modality".into())),
            }
        }
        Ok(self)
    }
}

#[derive(Debug, Clone)]
pub struct WeeklyDay {
    pub weekday: i16,
    pub routine_id: Option<Uuid>,
}

pub fn validate_week(days: &[WeeklyDay]) -> Result<(), DomainError> {
    let mut seen = [false; 7];
    if days.len() != 7 {
        return Err(DomainError::Validation(
            "provide exactly seven weekdays".into(),
        ));
    }
    for day in days {
        if !(1..=7).contains(&day.weekday)
            || seen[(day.weekday - 1) as usize]
            || day.routine_id.is_some_and(|id| id.is_nil())
        {
            return Err(DomainError::Validation(
                "weekdays must be unique values from 1 (Monday) to 7 (Sunday)".into(),
            ));
        }
        seen[(day.weekday - 1) as usize] = true;
    }
    Ok(())
}
