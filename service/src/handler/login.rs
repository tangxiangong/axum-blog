use axum::{extract::State, response::IntoResponse};
use common::{
    model::{AuthInfo, Claims, Login, LoginQuery},
    AppError, AppResponse,
};
use database::admin::get_password;
use sea_orm::DbConn;

/// 登录接口 API: `/login`
/// 默认不会返回 JWT, 除非用户选择了`记住我`，对应于 `/login?remember_me=true`
pub async fn signin(
    payload: LoginQuery,
    State(db_conn): State<DbConn>,
    admin: Login,
) -> impl IntoResponse {
    let exact_password = get_password(&admin.username, &db_conn).await?;
    if admin.password.ne(&exact_password) {
        return Err(AppError::unauth("密码错误"));
    }
    if payload.remember_me {
        let claims = Claims::new(&admin.username);
        let auth = claims.encode()?;
        Ok(AppResponse::<AuthInfo, ()>::data(auth))
    } else {
        Ok(AppResponse::ok())
    }
}
