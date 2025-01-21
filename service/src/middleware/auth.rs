use axum::{extract::Request, middleware::Next, response::Response};
use common::{model::Claims, AppResult};

/// JWT 认证中间件
pub async fn auth(claims: Claims, mut req: Request, next: Next) -> AppResult<Response> {
    let username = claims.username.clone();
    req.extensions_mut().insert(username);
    Ok(next.run(req).await)
}
