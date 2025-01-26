use crate::StateRouter;
use axum::Router;
use common::AppState;
use service::middleware::auth;

mod admin;

pub fn routes(state: AppState) -> StateRouter {
    Router::new()
        .merge(admin::routes())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth))
}
