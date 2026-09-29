use fitness_backend::domain::{
    errors::DomainError,
    user::{
        activity_level::ActivityLevel,
        entity::SignInUser,
        factory::{NewUserData, UserFactory},
        goal::Goal,
        profile::UserProfile,
        sex::Sex,
    },
};
use uuid::Uuid;

fn profile(weight: f32, height: i32, age: i32) -> Result<UserProfile, DomainError> {
    UserProfile::new(
        weight,
        height,
        age,
        ActivityLevel::ModeratelyActive,
        Goal::Maintain,
    )
}

#[test]
fn profile_rejects_invalid_values_and_accepts_existing_boundaries() {
    for weight in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -1.0, 0.0, 300.1] {
        assert!(profile(weight, 175, 30).is_err());
    }
    for height in [-1, 0, 251] {
        assert!(profile(75.0, height, 30).is_err());
    }
    for age in [-1, 0, 121] {
        assert!(profile(75.0, 175, age).is_err());
    }
    for (weight, height, age) in [(0.1, 1, 1), (300.0, 250, 120), (75.0, 175, 30)] {
        let profile = profile(weight, height, age).unwrap();
        assert_eq!(profile.weight(), weight);
        assert_eq!(profile.height(), height);
        assert_eq!(profile.age(), age);
    }
}

#[test]
fn user_factory_and_updates_preserve_validated_profiles_and_identity() {
    let mut user = UserFactory::create_user(NewUserData {
        username: "alice".into(),
        password_hash: "private-hash".into(),
        sex: Sex::Female,
        profile: profile(75.0, 175, 30).unwrap(),
    });
    let id = user.id();
    let updated =
        UserProfile::new(80.0, 180, 31, ActivityLevel::VeryActive, Goal::GainMuscle).unwrap();
    user.update_profile(updated.clone());
    assert_eq!(user.profile(), &updated);
    assert_eq!(user.id(), id);
    assert_eq!(user.username(), "alice");
    assert_eq!(user.sex(), Sex::Female);
    assert_eq!(user.password_hash(), "private-hash");
    assert_eq!(user.profile().activity_level(), ActivityLevel::VeryActive);
    assert_eq!(user.profile().goal(), Goal::GainMuscle);
}

#[test]
fn domain_debug_output_redacts_password_hashes() {
    let user = UserFactory::create_user(NewUserData {
        username: "alice".into(),
        password_hash: "private-hash".into(),
        sex: Sex::Male,
        profile: profile(75.0, 175, 30).unwrap(),
    });
    let sign_in = SignInUser::new(Uuid::new_v4(), "alice".into(), "private-hash".into());
    for output in [format!("{user:?}"), format!("{sign_in:?}")] {
        assert!(!output.contains("private-hash"));
        assert!(output.contains("[REDACTED]"));
    }
}
