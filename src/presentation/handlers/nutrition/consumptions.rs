use crate::{
    app_state::AppState,
    presentation::{
        authenticated_user::AuthenticatedUser,
        dto::{
            request::nutrition::{
                log_consumption_request::LogConsumptionRequest,
                update_consumption_request::UpdateConsumptionRequest,
            },
            response::nutrition::consumption_response::ConsumptionResponse,
        },
        errors::api_error::ApiError,
        extractors::{ApiJson, ApiPath},
        factories::response_factory::ResponseFactory,
    },
};
use axum::{
    extract::{Extension, State},
    response::Response,
};
use uuid::Uuid;

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiJson(request): ApiJson<LogConsumptionRequest>,
) -> Result<Response, ApiError> {
    let entry = state
        .log_consumption_use_case
        .execute(user.user_id, request.try_into()?)
        .await?;
    Ok(ResponseFactory::created(ConsumptionResponse::from(entry)))
}

pub async fn update(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(request): ApiJson<UpdateConsumptionRequest>,
) -> Result<Response, ApiError> {
    let entry = state
        .update_consumption_use_case
        .execute(user.user_id, id, request.try_into()?)
        .await?;
    Ok(ResponseFactory::ok(ConsumptionResponse::from(entry)))
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<Response, ApiError> {
    state
        .delete_consumption_use_case
        .execute(user.user_id, id)
        .await?;
    Ok(ResponseFactory::no_content())
}
