use axum::{routing::get, Router};
use service::handler::admin::{get_info, update_info};

use crate::StateRouter;

pub fn routes() -> StateRouter {
    Router::new().route("/admin", get(get_info).patch(update_info))
}
