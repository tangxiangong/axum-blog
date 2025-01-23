use axum::{extract::Request, middleware::Next, response::Response};
use common::{
    model::{Claims, RedisConn, Session},
    AppError, AppResult,
};
use redis::AsyncCommands;

/// JWT 与 Session 认证中间件
pub async fn auth(
    claims: Option<Claims>,
    session: Option<Session>,
    RedisConn(mut conn): RedisConn,
    req: Request,
    next: Next,
) -> AppResult<Response> {
    // req.extensions_mut().insert(_claims.unwrap().username);
    if claims.is_none() && session.is_none() {
        return Err(AppError::unauth("认证失败, 请重新登录"));
    }
    if session.is_some() {
        session.unwrap().update(conn).await?;
    } else {
        // claims == Some
        let token = claims.unwrap().encode()?;
        if conn.exists(&token).await? {
            return Err(AppError::unauth("认证失败, 请重新登录"));
        }
    }
    Ok(next.run(req).await)
}
