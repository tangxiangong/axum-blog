use axum::http::Method;
use axum::{
    routing::{get, post},
    Router,
};
use service::{handler::signin, middleware::auth, middleware::timing};
use setting::AppState;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub type StateRouter = Router<AppState>;

pub fn compose(state: AppState) -> StateRouter {
    Router::new()
        .route("/api", get(|| async { "Hello, World!" }))
        .route("/signin", post(signin))
        .route(
            "/protected",
            get(|| async { "hello" })
                .route_layer(axum::middleware::from_fn_with_state(state.clone(), auth)),
        )
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
