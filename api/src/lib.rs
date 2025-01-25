use axum::{
    routing::{get, post},
    Router,
};
use service::{handler::signin, middleware::global};
use setting::AppState;

mod protected;
mod upload;

pub type StateRouter = Router<AppState>;

pub fn compose(state: AppState) -> StateRouter {
    Router::new()
        .route("/api", get(|| async { "Hello, World!" }))
        .route("/signin", post(signin))
        .merge(protected::routes(state.clone()))
        .merge(upload::routes())
        .merge(global())
}
