use crate::StateRouter;
use axum::{routing::get, Router};
use service::handler::protected::test as test_handler;

pub fn test() -> StateRouter {
    Router::new().route("/test", get(test_handler))
}
