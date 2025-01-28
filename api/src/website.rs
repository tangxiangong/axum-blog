use crate::StateRouter;
use axum::{
    Router,
    routing::{get, patch},
};
use service::handler::website::*;

pub fn public_routes() -> StateRouter {
    Router::new().route("/website", get(info))
}

pub fn protected_routes() -> StateRouter {
    Router::new()
        .route("/website", patch(update_info))
        .route("/website/upload/logo", patch(update_logo))
        .route("/website/upload/favicon", patch(update_favicon))
}
