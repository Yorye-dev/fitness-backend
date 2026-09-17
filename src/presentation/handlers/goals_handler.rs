use axum::{
    extract::{State, Extension},
    response::IntoResponse,
};
use crate::services::Services;
use crate::auth::claims::Claims;
use crate::presentation::factories::api_response_factory::ResponseFactory;

pub async fn stats_handler (
    State(_services): State<Services>,
    Extension(_claims): Extension<Claims>,
) -> impl IntoResponse 
{
    ResponseFactory::ok("Stats endpoint - to be implemented")
}
