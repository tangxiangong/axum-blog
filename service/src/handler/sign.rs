use axum::{
    http::{header::SET_COOKIE, HeaderValue},
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use common::{
    model::{Claims, Login, MySQLConn, RedisConn, RedisPoolConn, RememberMe, Session},
    AppError, AppResponse, AppResult,
};
use database::admin::get_password;

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
    if payload.remember_me {
        let claims = Claims::new(&admin.username);
        let token = claims.encode()?;
        // let bearer = format!("Bearer {}", auth.token);
        let token = HeaderValue::from_str(&token).map_err(|e| AppError::internal(e.to_string()))?;
        res.headers_mut().insert("Bearer", token);
        Ok(res)
    } else {
        Ok(res)
    }
}

/// 登出
/// 1. 删除 Redis 中的 Session
/// 2. 将 JWT 加入黑名单
#[allow(unused_variables)]
pub async fn signout(
    MySQLConn(db_conn): MySQLConn,
    RedisConn(redis_conn): RedisConn,
    claims: Option<Claims>,
    session: Option<Session>,
    admin: Login,
) -> AppResult<()> {
    todo!()
}

async fn create_session(value: &str, conn: RedisPoolConn) -> AppResult<String> {
    let mut session = Session::default();
    session.insert("username", value)?;
    session.save(conn).await?;
    let id = session.id().to_string();
    Ok(id)
}
