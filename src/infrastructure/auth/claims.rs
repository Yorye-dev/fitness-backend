use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum TokenType {
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct Claims {
    pub(super) subject: String,
    pub(super) exp: usize,
    pub(super) token_type: TokenType,
}

impl Claims {
    pub(super) fn new(user_id: &str, expiration_minutes: i64, token_type: TokenType) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(expiration_minutes))
            .expect("valid timestamp")
            .timestamp() as usize;

        Claims {
            subject: user_id.to_string(),
            exp: expiration,
            token_type,
        }
    }
}
