use axum::{extract::Request, middleware::Next, response::Response};
use common::{
    AppError, AppResult, RedisConn,
    model::{Claims, Session},
};
use redis::AsyncCommands;

/// JWT 与 Session 认证中间件
pub async fn auth(
    claims: Option<Claims>,
    session_option: Option<Session>,
    RedisConn(mut conn): RedisConn,
    mut req: Request,
    next: Next,
) -> AppResult<Response> {
    // req.extensions_mut().insert(_claims.unwrap().username);
    if claims.is_none() && session_option.is_none() {
        return Err(AppError::unauth("认证失败, 请重新登录"));
    }
    let uid = if let Some(mut session) = session_option {
        req.extensions_mut().insert(session.clone());
        session.update(conn).await?;
        session.get("uid")?.unwrap()
    } else {
        // claims == Some
        let claims = claims.unwrap();
        req.extensions_mut().insert(claims.clone());
        let token = claims.encode()?;
        // token 在黑名单中
        if conn.exists(&token).await? {
            return Err(AppError::unauth("认证失败, 请重新登录"));
        }
        claims.uid.to_owned()
    };
    req.extensions_mut().insert(uid);
    Ok(next.run(req).await)
}
