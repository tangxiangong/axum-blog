use crate::{admin, website, StateRouter};
use axum::Router;
use common::AppState;
use service::middleware::auth;

pub fn routes(state: AppState) -> StateRouter {
    Router::new()
        .merge(admin::protected_routes())
        .merge(website::protected_routes())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth))
}
