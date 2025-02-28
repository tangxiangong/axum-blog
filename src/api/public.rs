use crate::api::{StateRouter, admin, website};
use axum::Router;

pub fn routes() -> StateRouter {
    Router::new()
        .merge(admin::public_routes())
        .merge(website::public_routes())
}
