use serde::Serialize;

use super::pagination::PaginationMeta;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ApiMeta>,
}

#[derive(Debug, Serialize)]
pub struct ApiMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMeta>,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            meta: None,
        }
    }

    pub fn paginated(
        data: T,
        pagination: PaginationMeta,
    ) -> Self {
        Self {
            data,
            meta: Some(ApiMeta {
                pagination: Some(pagination),
            }),
        }
    }
}
