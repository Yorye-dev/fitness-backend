use serde::Serialize;
use super::pagination::PaginationMeta;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ApiMeta>,
}

#[derive(Serialize)]
pub struct ApiMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMeta>,
}
