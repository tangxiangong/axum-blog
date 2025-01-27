use axum::Router;
use common::AppState;
use service::middleware::global;

mod protected;
mod public;

mod admin;

pub type StateRouter = Router<AppState>;

pub fn compose(state: AppState) -> StateRouter {
    Router::new()
        .merge(public::routes())
        .merge(protected::routes(state.clone()))
        .merge(global())
}
