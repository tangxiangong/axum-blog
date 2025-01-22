use axum::{extract::Request, middleware::Next};
use common::{
    model::{Claims, Session},
    AppError, AppResult,
};

/// JWT 与 Session 认证中间件
/// TODO 利用 Redis 实现 Session 机制 Session 和 Claims 均实现 `OptionalFromRequestParts`
pub async fn auth(
    claims: Option<Claims>,
    session: Option<Session>,
    req: Request,
    next: Next,
) -> AppResult {
    // req.extensions_mut().insert(_claims.unwrap().username);
    if claims.is_none() && session.is_none() {
        return Err(AppError::unauth("请登录"));
    }
    next.run(req).await;
    session.unwrap().update();
    Ok(())
}
