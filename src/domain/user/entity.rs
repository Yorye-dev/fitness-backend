use super::{profile::UserProfile, sex::Sex};
use std::fmt;
use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    id: Uuid,
    username: String,
    password_hash: String,
    sex: Sex,
    profile: UserProfile,
}

impl User {
    /// Constructs or restores a user with an already validated profile.
    pub fn new(
        id: Uuid,
        username: String,
        password_hash: String,
        sex: Sex,
        profile: UserProfile,
    ) -> Self {
        Self {
            id,
            username,
            password_hash,
            sex,
            profile,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn sex(&self) -> Sex {
        self.sex
    }

    pub fn profile(&self) -> &UserProfile {
        &self.profile
    }

    pub fn update_profile(&mut self, profile: UserProfile) {
        self.profile = profile;
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password_hash", &"[REDACTED]")
            .field("sex", &self.sex)
            .field("profile", &self.profile)
            .finish()
    }
}

#[derive(Clone)]
pub struct SignInUser {
    id: Uuid,
    username: String,
    password_hash: String,
}

impl SignInUser {
    pub fn new(id: Uuid, username: String, password_hash: String) -> Self {
        Self {
            id,
            username,
            password_hash,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
}

impl fmt::Debug for SignInUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SignInUser")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password_hash", &"[REDACTED]")
            .finish()
    }
}
