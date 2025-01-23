use crate::StateRouter;
use axum::{routing::get, Router};
use service::middleware::auth;
use setting::AppState;

pub fn routes(state: AppState) -> StateRouter {
    Router::new()
        .route("/protected", get(|| async { "hello" }))
        .route("/signout", get(|| async { "注销成功" }))
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth))
}
