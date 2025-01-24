use crate::StateRouter;
use axum::{routing::get, Router};
use service::{handler::signout, middleware::auth};
use setting::AppState;

mod test;
use test::test;

pub fn routes(state: AppState) -> StateRouter {
    let root = Router::new().route("/protected", get(|| async { "hello" }));
    root.nest("/protected", test())
        .route("/signout", get(signout))
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth))
}
