use crate::{AppResult, DatabaseClient, database::common::next_id, model::Claims};
use chrono::Local;
use serde::Deserialize;
use surrealdb::types::SurrealValue;

#[derive(Debug, Deserialize, SurrealValue)]
struct JwtTokenRow {
    token: String,
    expire_at: i64,
}

pub async fn add(uid: &str, db_conn: &DatabaseClient) -> AppResult<String> {
    let claims = Claims::new(uid);
    let token = claims.encode()?;
    let duration = claims.exp_secs();
    let expire_at = claims.exp as i64;
    let id = next_id("jwt", db_conn).await?;

    db_conn
        .query(
            "CREATE type::thing('jwt', $rid) CONTENT {
                id: $id,
                user_uuid: $uid,
                token: $token,
                expire_duration: $duration,
                expire_at: $expire_at,
                created_at: time::now(),
                updated_at: time::now()
            };",
        )
        .bind(("rid", id))
        .bind(("id", id))
        .bind(("uid", uid.to_owned()))
        .bind(("token", token.clone()))
        .bind(("duration", duration))
        .bind(("expire_at", expire_at))
        .await?;

    Ok(token)
}

pub async fn find_by_uid(uid: &str, db_conn: &DatabaseClient) -> AppResult<Vec<(String, i64)>> {
    let current = Local::now().timestamp();
    let mut response = db_conn
        .query(
            "SELECT token, expire_at
             FROM jwt
             WHERE user_uuid = $uid
               AND expire_at > $current
             ORDER BY expire_at ASC;",
        )
        .bind(("uid", uid.to_owned()))
        .bind(("current", current))
        .await?;

    let rows: Vec<JwtTokenRow> = response.take(0)?;
    Ok(rows
        .into_iter()
        .map(|row| (row.token, row.expire_at))
        .collect())
}
