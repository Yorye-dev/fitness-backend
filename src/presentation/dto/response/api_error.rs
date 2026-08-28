use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub error: ApiErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorDetail {
    pub code: &'static str,
    pub message: String,
}

impl ApiErrorResponse {
    pub fn new(
        code: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            error: ApiErrorDetail {
                code,
                message: message.into(),
            },
        }
    }
}
