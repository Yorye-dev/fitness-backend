use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignInData {
    pub password: String,  // Password entered during sign-in
}

