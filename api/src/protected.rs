use crate::StateRouter;
use axum::{routing::get, Router};
use common::AppState;
use service::{handler::signout, middleware::auth};

mod test;
use test::test;

mod admin;

pub fn routes(state: AppState) -> StateRouter {
    let root = Router::new().route("/protected", get(|| async { "hello" }));
    root.nest("/protected", test())
        .route("/signout", get(signout))
        .merge(admin::routes())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth))
}
