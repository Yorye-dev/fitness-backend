use crate::domain::errors::RepositoryError;
use crate::domain::user::{
    activity_level::ActivityLevel,
    entity::{SignInUser, User},
    goal::Goal,
    profile::UserProfile,
    sex::Sex,
};
use crate::infrastructure::persistence::models::user_row::{
    ActivityLevelRow, GoalRow, SexRow, SignInUserRow, UserRow,
};

macro_rules! map_enum {
    ($domain:ty, $row:ty, $($variant:ident),+ $(,)?) => {
        impl From<$row> for $domain {
            fn from(row: $row) -> Self {
                match row { $(<$row>::$variant => Self::$variant),+ }
            }
        }
        impl From<$domain> for $row {
            fn from(value: $domain) -> Self {
                match value { $(<$domain>::$variant => Self::$variant),+ }
            }
        }
    };
}
map_enum!(Sex, SexRow, Male, Female);
map_enum!(Goal, GoalRow, LoseWeight, Maintain, GainMuscle);
map_enum!(
    ActivityLevel,
    ActivityLevelRow,
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    ExtraActive
);

impl TryFrom<UserRow> for User {
    type Error = RepositoryError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        let profile = UserProfile::new(
            row.weight,
            row.height,
            row.age,
            row.activity_level.into(),
            row.goal.into(),
        )
        .map_err(|_| RepositoryError::Unexpected)?;
        Ok(Self::new(
            row.id,
            row.username,
            row.password_hash,
            row.sex.into(),
            profile,
        ))
    }
}
impl From<SignInUserRow> for SignInUser {
    fn from(row: SignInUserRow) -> Self {
        Self::new(row.id, row.username, row.password_hash)
    }
}
