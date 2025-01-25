use axum::{routing::post, Router};
use service::handler::upload::upload_image;

use crate::StateRouter;

pub fn routes() -> StateRouter {
    Router::new().route("/upload/test", post(upload_image))
}
