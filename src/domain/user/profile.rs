use super::{activity_level::ActivityLevel, goal::Goal};
use crate::domain::errors::DomainError;

/// Validated profile data shared by registration, updates and persistence.
#[derive(Debug, Clone, PartialEq)]
pub struct UserProfile {
    weight: f32,
    height: i32,
    age: i32,
    activity_level: ActivityLevel,
    goal: Goal,
}

impl UserProfile {
    pub fn new(
        weight: f32,
        height: i32,
        age: i32,
        activity_level: ActivityLevel,
        goal: Goal,
    ) -> Result<Self, DomainError> {
        if !weight.is_finite()
            || weight <= 0.0
            || weight > 300.0
            || !(1..=250).contains(&height)
            || !(1..=120).contains(&age)
        {
            return Err(DomainError::Validation(
                "invalid weight, height or age".into(),
            ));
        }
        Ok(Self {
            weight,
            height,
            age,
            activity_level,
            goal,
        })
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn age(&self) -> i32 {
        self.age
    }

    pub fn activity_level(&self) -> ActivityLevel {
        self.activity_level
    }

    pub fn goal(&self) -> Goal {
        self.goal
    }
}
