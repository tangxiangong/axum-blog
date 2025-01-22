use axum::{extract::Request, middleware::Next, response::Response};
use common::model::Claims;

/// JWT 与 Session 认证中间件
/// TODO 利用 Redis 实现 Session 机制 Session 和 Claims 均实现 `OptionalFromRequestParts`
pub async fn auth(_claims: Option<Claims>, req: Request, next: Next) -> Response {
    // req.extensions_mut().insert(_claims.unwrap().username);
    next.run(req).await
}
