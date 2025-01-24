use axum::{
    http::{header::SET_COOKIE, HeaderValue},
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use common::{
    model::{Claims, Login, MySQLConn, RedisConn, RedisPoolConn, RememberMe, Session},
    // utils::jwt::get_jwt_payload,
    AppError,
    AppResponse,
    AppResult,
};
use database::admin::get_password;
use redis::AsyncCommands;

/// 登录接口 API: `/login`
/// 默认不会返回 JWT, 除非用户选择了`记住我`，对应于 `/login?remember_me=true`
pub async fn signin(
    payload: RememberMe,
    MySQLConn(db_conn): MySQLConn,
    RedisConn(redis_conn): RedisConn,
    admin: Login,
) -> impl IntoResponse {
    let exact_password = get_password(&admin.username, &db_conn).await?;
    if admin.password.ne(&exact_password) {
        return Err(AppError::unauth("密码错误"));
    }
    let mut res = AppResponse::ok().into_response();
    // let mut token: Option<String> = None;
    if payload.remember_me {
        let claims = Claims::new(&admin.username);
        let enc_str = claims.encode()?;
        // token = Some(enc_str.clone());
        // let bearer = format!("Bearer {}", auth.token);
        let token_value =
            HeaderValue::try_from(&enc_str).map_err(|e| AppError::internal(e.to_string()))?;
        res.headers_mut().insert("Bearer", token_value);
    }
    // let payload = token.map(|v| get_jwt_payload(&v));
    // if let Ok(id) = create_session(&admin.username, payload, redis_conn).await {
    if let Ok(id) = create_session(&admin.username, redis_conn).await {
        let cookie = Cookie::build(("JSESSIONID", &id))
            .http_only(true)
            .same_site(SameSite::Strict)
            .secure(true)
            .to_string();
        if let Ok(cookie_value) = HeaderValue::try_from(&cookie) {
            res.headers_mut().insert(SET_COOKIE, cookie_value);
        }
    }
    Ok(res)
}

/// 登出
/// 1. 删除 Redis 中的 Session
/// 2. 将 JWT 加入黑名单(Redis)
#[allow(unused_variables)]
pub async fn signout(
    MySQLConn(db_conn): MySQLConn,
    RedisConn(mut redis_conn): RedisConn,
    claims: Option<Claims>,
    session: Option<Session>,
    admin: Login,
) -> AppResult<()> {
    if let Some(session) = session {
        let session_id = session.id().to_string();
        // if let Some(payload) = session.payload() {}
        if redis_conn.exists(&session_id).await? {
            let _: () = redis_conn.del(session_id).await?;
        }
    }

    if let Some(claims) = claims {
        let token = claims.encode()?;
        let secs = claims.exp_secs();
        if !redis_conn.exists(&token).await? {
            let _: () = redis_conn.set(&token, "invalid").await?;
            let _: () = redis_conn.expire(&token, secs).await?;
        }
    }

    Ok(())
}

async fn create_session(
    value: &str,
    // payload: Option<String>,
    conn: RedisPoolConn,
) -> AppResult<String> {
    // let mut session = Session::with_payload(payload);
    let mut session = Session::default();
    session.insert("username", value)?;
    session.save(conn).await?;
    let id = session.id().to_string();
    Ok(id)
}
