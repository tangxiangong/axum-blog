use axum::{extract::Request, middleware::Next, response::Response};
use common::model::Claims;

/// JWT 认证中间件
pub async fn auth(_claims: Claims, req: Request, next: Next) -> Response {
    next.run(req).await
}
