use crate::{
    AppError, AppResponse, AppResult, DbConn, RedisClient, RedisConn,
    database::{
        admin::{get_password, get_uid},
        jwt::add,
    },
    model::{Claims, Login, RememberMe, Session},
};
use axum::{
    extract::Extension,
    http::{HeaderValue, header::SET_COOKIE},
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use redis::AsyncCommands;

/// 登录接口 API: `/login`
/// 默认不会返回 JWT, 除非用户选择了`记住我`，对应于 `/login?remember_me=true`
pub async fn signin(
    payload: RememberMe,
    DbConn(db_conn): DbConn,
    RedisConn(redis_conn): RedisConn,
    admin: Login,
) -> impl IntoResponse {
    let exact_password = get_password(&admin.username, &db_conn).await?;
    if admin.password.ne(&exact_password) {
        return Err(AppError::unauth("密码错误"));
    }
    let uid = get_uid(&admin.username, &db_conn).await?;
    let mut res = AppResponse::ok().into_response();
    // let mut token: Option<String> = None;
    if payload.remember_me {
        let token = add(&uid, &db_conn).await?;
        // token = Some(enc_str.clone());
        // let bearer = format!("Bearer {}", auth.token);
        let token_value =
            HeaderValue::try_from(&token).map_err(|e| AppError::internal(e.to_string()))?;
        res.headers_mut().insert("Bearer", token_value);
    }

    if let Ok(session_id) = create_session(&uid, redis_conn).await {
        let cookie = Cookie::build(("SESSION_ID", &session_id))
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
pub async fn signout(
    RedisConn(mut redis_conn): RedisConn,
    claims: Extension<Option<Claims>>,
    session: Extension<Option<Session>>,
) -> AppResult {
    if let Some(session) = session.0 {
        let session_id = session.id().to_string();
        // if let Some(payload) = session.payload() {}
        if redis_conn.exists(&session_id).await? {
            let _: () = redis_conn.del(session_id).await?;
        }
    }

    if let Some(claims) = claims.0 {
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
    conn: RedisClient,
) -> AppResult<String> {
    // let mut session = Session::with_payload(payload);
    let mut session = Session::default();
    session.insert("uid", value)?;
    session.save(conn).await?;
    let id = session.id().to_string();
    Ok(id)
}
