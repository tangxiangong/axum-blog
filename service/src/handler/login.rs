use axum::{extract::State, http::HeaderValue, response::IntoResponse};
use common::{
    model::{Claims, Login, RememberMe},
    AppError, AppResponse,
};
use database::admin::get_password;
use sea_orm::DbConn;

// TODO 利用 Redis 实现 Session 机制

/// 登录接口 API: `/login`
/// 默认不会返回 JWT, 除非用户选择了`记住我`，对应于 `/login?remember_me=true`
pub async fn signin(
    payload: RememberMe,
    State(db_conn): State<DbConn>,
    admin: Login,
) -> impl IntoResponse {
    let exact_password = get_password(&admin.username, &db_conn).await?;
    if admin.password.ne(&exact_password) {
        return Err(AppError::unauth("密码错误"));
    }
    let mut res = AppResponse::ok().into_response();
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
