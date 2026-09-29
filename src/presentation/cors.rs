use axum::http::{HeaderValue, Method, header::InvalidHeaderValue};
use tower_http::cors::{Any, CorsLayer};

pub fn cors_layer(origins: &[String]) -> Result<CorsLayer, InvalidHeaderValue> {
    let origins = origins
        .iter()
        .map(|origin| origin.parse::<HeaderValue>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any))
}
