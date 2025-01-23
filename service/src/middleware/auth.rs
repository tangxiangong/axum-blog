use axum::{extract::Request, middleware::Next};
use common::{
    model::{Claims, RedisConn, Session},
    AppError, AppResult,
};

/// JWT 与 Session 认证中间件
pub async fn auth(
    claims: Option<Claims>,
    session: Option<Session>,
    RedisConn(conn): RedisConn,
    req: Request,
    next: Next,
) -> AppResult {
    // req.extensions_mut().insert(_claims.unwrap().username);
    if claims.is_none() && session.is_none() {
        return Err(AppError::unauth("请登录"));
    }
    session.unwrap().update(conn).await?;
    next.run(req).await;
    Ok(())
}
