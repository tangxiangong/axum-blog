use crate::middleware::timing;
use axum::{http::Method, Router};
use setting::AppState;
use std::time::Duration;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub fn global() -> Router<AppState> {
    Router::new()
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(axum::middleware::from_fn(timing))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http())
}
