use axum::{
    routing::{get, patch},
    Router,
};
use service::handler::{admin::*, signout};

use crate::StateRouter;

pub fn routes() -> StateRouter {
    Router::new()
        .route("/signout", get(signout))
        .route("/admin", get(get_info).patch(update_info))
        .route("/admin/upload", patch(update_avatar))
}
