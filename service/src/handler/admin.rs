use super::{signout, utils};
use axum::Extension;
use chrono::Local;
use common::{
    entity::Admin,
    model::{Claims, Image, Session, UpdateAdminInfo},
    AppResponse, AppResponseResult, AppResult, MySQLConn, RedisConn, RedisPoolConn,
};
use database::admin as db;
use database::jwt::find_by_uid;
use redis::AsyncCommands;

pub async fn info(
    MySQLConn(db_conn): MySQLConn,
    Extension(uid): Extension<String>,
) -> AppResponseResult<Admin> {
    let admin = db::get_info(&uid, &db_conn).await?;
    Ok(AppResponse::data(admin))
}

pub async fn update_info(
    MySQLConn(db_conn): MySQLConn,
    Extension(uid): Extension<String>,
    info: UpdateAdminInfo,
) -> AppResult<()> {
    db::update_info(&uid, info, &db_conn).await?;
    Ok(())
}

pub async fn update_avatar(
    MySQLConn(db_conn): MySQLConn,
    Extension(uid): Extension<String>,
    avatar: Image,
) -> AppResult<()> {
    let path = utils::save_image(avatar).await?;
    db::update_avatar(&uid, &path, &db_conn).await?;
    Ok(())
}

/// 更新密码
/// 将该用户的所有有效期内的 JWT 加入 Redis 黑名单
pub async fn update_pwd(
    MySQLConn(db_conn): MySQLConn,
    RedisConn(mut redis_conn): RedisConn,
    session: Option<Extension<Session>>,
    claims: Option<Extension<Claims>>,
    uid: Extension<String>,
    pwd: String,
) -> AppResult<()> {
    db::update_pwd(&uid, &pwd, &db_conn).await?;
    let invalid_tokens = find_by_uid(&uid, &db_conn).await?;
    jwt_blacklist(invalid_tokens, &mut redis_conn).await?;
    signout(RedisConn(redis_conn), claims, session).await
}

async fn jwt_blacklist(tokens: Vec<(String, i64)>, conn: &mut RedisPoolConn) -> AppResult {
    let current_timestamp = Local::now().timestamp();
    for (ref token, exp_at) in tokens {
        if !conn.exists(token).await? {
            let _: () = conn.set(token, "invalid").await?;
        }
        let exp = exp_at - current_timestamp;
        let _: () = conn.expire(token, exp).await?;
    }
    Ok(())
}
