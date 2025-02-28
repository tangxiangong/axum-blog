use crate::{
    AppState,
    api::{StateRouter, admin, website},
    service::middleware::auth,
};
use axum::Router;

pub fn routes(state: AppState) -> StateRouter {
    Router::new()
        .merge(admin::protected_routes())
        .merge(website::protected_routes())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth))
}
