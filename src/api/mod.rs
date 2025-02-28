use crate::{AppState, service::middleware::global};
use axum::Router;

mod protected;
mod public;

mod admin;
mod website;

pub type StateRouter = Router<AppState>;

pub fn compose(state: AppState) -> StateRouter {
    Router::new()
        .merge(public::routes())
        .merge(protected::routes(state.clone()))
        .merge(global())
}
