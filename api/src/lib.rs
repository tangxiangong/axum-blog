use axum::http::Method;
use axum::{routing::get, Router};
use sea_orm::DbConn;
use service::middleware::timing;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub type StateRouter = Router<DbConn>;

pub fn compose() -> StateRouter {
    Router::new()
        .route("/api", get(|| async { "Hello, World!" }))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST])
                        .allow_origin(Any),
                )
                .layer(axum::middleware::from_fn(timing))
                .layer(TimeoutLayer::new(Duration::from_secs(10))),
        )
}
