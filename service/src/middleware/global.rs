use crate::middleware::timing;
use axum::{extract::DefaultBodyLimit, http::Method, Router};
use common::AppState;
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
        // 限制请求体最大为 10 MB
        .layer(DefaultBodyLimit::max(1024 * 1024 * 10))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http())
}
