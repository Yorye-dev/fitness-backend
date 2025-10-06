use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignInData {
    pub username: String,
    pub password: String,  // Password entered during sign-in
}

