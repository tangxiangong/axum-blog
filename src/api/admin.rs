use crate::{
    api::StateRouter,
    service::handler::{admin::*, signin, signout},
};
use axum::{
    Router,
    routing::{get, patch, post},
};

pub fn public_routes() -> StateRouter {
    Router::new().route("/signin", post(signin))
}

pub fn protected_routes() -> StateRouter {
    Router::new()
        .route("/signout", post(signout))
        .route("/admin", get(info))
        .route("/admin", patch(update_info))
        .route("/admin/upload", patch(update_avatar))
}
