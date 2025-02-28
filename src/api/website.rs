use crate::{api::StateRouter, service::handler::website::*};
use axum::{
    Router,
    routing::{get, patch},
};

pub fn public_routes() -> StateRouter {
    Router::new().route("/website", get(info))
}

pub fn protected_routes() -> StateRouter {
    Router::new()
        .route("/website", patch(update_info))
        .route("/website/upload/logo", patch(update_logo))
        .route("/website/upload/favicon", patch(update_favicon))
}
