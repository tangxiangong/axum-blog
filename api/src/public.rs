use crate::{admin, StateRouter};
use axum::Router;

pub fn routes() -> StateRouter {
    Router::new().merge(admin::public_routes())
}
