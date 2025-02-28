use crate::{
    api::StateRouter,
    service::handler::{admin::*, signin, signout},
};
use axum::{
    Router,
    routing::{get, patch, post},
};

pub fn public_routes() -> StateRouter {
    Router::new()
        .route("/signin", post(signin))
        .route("/admin", get(info))
}

pub fn protected_routes() -> StateRouter {
    Router::new()
        .route("/signout", post(signout))
        .route("/admin", patch(update_info))
        .route("/admin/upload", patch(update_avatar))
}
