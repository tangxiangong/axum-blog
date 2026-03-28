use crate::{AppError, AppResult, DatabaseClient};
use surrealdb::{IndexedResults, types::SurrealValue};

pub fn take_one<T: SurrealValue>(
    response: &mut IndexedResults,
    index: usize,
) -> AppResult<Option<T>> {
    let rows: Vec<T> = response.take(index)?;
    Ok(rows.into_iter().next())
}

pub async fn next_id(table: &str, db_conn: &DatabaseClient) -> AppResult<u32> {
    let mut response = db_conn
        .query(
            "UPSERT type::thing('counter', $table) \
             SET value = IF value = NONE THEN 1 ELSE value + 1 END \
             RETURN VALUE value;",
        )
        .bind(("table", table.to_string()))
        .await?;

    let values: Vec<u32> = response.take(0)?;
    values
        .into_iter()
        .next()
        .ok_or_else(|| AppError::internal(format!("无法生成 {} 的自增 ID", table)))
}
