use chrono::{Local, TimeZone};
use common::{
    entity::{jwt, ActiveJwt, JwtEntity},
    model::Claims,
    AppResult,
};
use sea_orm::{prelude::*, DbConn, QuerySelect, Set};

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
    Ok(JwtEntity::find()
        .select_only()
        .columns([jwt::Column::Token, jwt::Column::ExpireAt])
        .filter(jwt::Column::UserUuid.eq(uid))
        .filter(jwt::Column::ExpireAt.gt(Local::now().timestamp()))
        .into_tuple::<(String, i64)>()
        .all(db_conn)
        .await?)
}
