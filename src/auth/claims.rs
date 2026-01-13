use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Claims {
    pub subject: String,
    pub exp: usize
}

impl Claims {
    
    pub fn new(user_id: &str, expiration_minutes: i64) -> Self {
        
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(expiration_minutes))
            .expect("valid timestamp")
            .timestamp() as usize;

        Claims { subject: user_id.to_string(), exp: expiration }
    }
}
