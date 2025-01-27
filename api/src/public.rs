use crate::{admin, website, StateRouter};
use axum::Router;

pub fn routes() -> StateRouter {
    Router::new()
        .merge(admin::public_routes())
        .merge(website::public_routes())
}
