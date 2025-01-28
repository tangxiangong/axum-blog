use chrono::{Local, TimeZone};
use common::{
    AppResult,
    entity::{ActiveJwt, JwtEntity, jwt},
    model::Claims,
};
use sea_orm::{DbConn, QuerySelect, Set, prelude::*};

pub async fn add(uid: &str, db_conn: &DbConn) -> AppResult<String> {
    let claims = Claims::new(uid);
    let token = claims.encode()?;
    let duration = claims.exp_secs();
    let expire_at = Local.timestamp_opt(claims.exp as i64, 0).unwrap();
    ActiveJwt {
        token: Set(token.clone()),
        user_uuid: Set(uid.to_string()),
        expire_duration: Set(duration),
        expire_at: Set(expire_at),
        ..Default::default()
    }
    .insert(db_conn)
    .await?;
    Ok(token)
}

pub async fn find_by_uid(uid: &str, db_conn: &DbConn) -> AppResult<Vec<(String, i64)>> {
    let current = Local::now().timestamp();
    Ok(JwtEntity::find()
        .select_only()
        .columns([jwt::Column::Token, jwt::Column::ExpireAt])
        .filter(jwt::Column::UserUuid.eq(uid))
        .filter(jwt::Column::ExpireAt.gt(current))
        .into_tuple::<(String, i64)>()
        .all(db_conn)
        .await?)
}
