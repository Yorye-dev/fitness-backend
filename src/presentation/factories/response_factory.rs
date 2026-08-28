use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
    Json,
};
use serde::Serialize;

use crate::presentation::dto::response::{
    api_response::ApiResponse,
    pagination::PaginationMeta,
};

pub struct ResponseFactory;

impl ResponseFactory {
    pub fn ok<T>(data: T) -> Response
    where
        T: Serialize,
    {
        Self::success(
            StatusCode::OK,
            data,
        )
    }

    pub fn created<T>(data: T) -> Response
    where
        T: Serialize,
    {
        Self::success(
            StatusCode::CREATED,
            data,
        )
    }

    pub fn success<T>(
        status: StatusCode,
        data: T,
    ) -> Response
    where
        T: Serialize,
    {
        (
            status,
            Json(ApiResponse::new(data)),
        )
            .into_response()
    }

    pub fn paginated<T>(
        data: Vec<T>,
        page: u32,
        per_page: u32,
        total: u64,
    ) -> Response
    where
        T: Serialize,
    {
        let pagination = PaginationMeta::new(
            page,
            per_page,
            total,
        );

        let response = ApiResponse::paginated(
            data,
            pagination,
        );

        (
            StatusCode::OK,
            Json(response),
        )
            .into_response()
    }

    pub fn no_content() -> Response {
        StatusCode::NO_CONTENT.into_response()
    }
}
