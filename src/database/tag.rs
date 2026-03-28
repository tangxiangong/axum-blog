use crate::{
    AppError, AppResult, DatabaseClient,
    database::common::{next_id, take_one},
    entity::Tag,
};

pub async fn add(tag_name: &str, db_conn: &DatabaseClient) -> AppResult {
    if find_by_name(tag_name, db_conn).await?.is_some() {
        return Err(AppError::bad_request("标签已存在"));
    }

    let id = next_id("tag", db_conn).await?;
    db_conn
        .query(
            "CREATE type::thing('tag', $rid) CONTENT {
                id: $id,
                name: $name,
                created_at: time::now(),
                updated_at: time::now()
            };",
        )
        .bind(("rid", id))
        .bind(("id", id))
        .bind(("name", tag_name.to_owned()))
        .await?;

    Ok(())
}

pub async fn find_by_id(id: u32, db_conn: &DatabaseClient) -> AppResult<Option<Tag>> {
    let mut response = db_conn
        .query(
            "SELECT id, name, created_at, updated_at
             FROM tag
             WHERE id = $id
             LIMIT 1;",
        )
        .bind(("id", id))
        .await?;

    take_one(&mut response, 0)
}

pub async fn update(tag_id: u32, new_tag_name: &str, db_conn: &DatabaseClient) -> AppResult {
    if find_by_id(tag_id, db_conn).await?.is_none() {
        return Err(AppError::bad_request("标签不存在"));
    }

    if let Some(existing) = find_by_name(new_tag_name, db_conn).await?
        && existing.id != tag_id
    {
        return Err(AppError::bad_request("标签已存在"));
    }

    db_conn
        .query(
            "UPDATE tag
             SET name = $name,
                 updated_at = time::now()
             WHERE id = $id;",
        )
        .bind(("id", tag_id))
        .bind(("name", new_tag_name.to_owned()))
        .await?;

    Ok(())
}

pub async fn find_by_name(name: &str, db_conn: &DatabaseClient) -> AppResult<Option<Tag>> {
    let mut response = db_conn
        .query(
            "SELECT id, name, created_at, updated_at
             FROM tag
             WHERE name = $name
             LIMIT 1;",
        )
        .bind(("name", name.to_owned()))
        .await?;

    take_one(&mut response, 0)
}

pub async fn delete_by_id(id: u32, db_conn: &DatabaseClient) -> AppResult {
    if find_by_id(id, db_conn).await?.is_none() {
        return Err(AppError::bad_request("标签不存在"));
    }

    db_conn
        .query("DELETE tag WHERE id = $id;")
        .bind(("id", id))
        .await?;

    Ok(())
}

pub async fn delete_by_name(name: &str, db_conn: &DatabaseClient) -> AppResult {
    let id = match find_by_name(name, db_conn).await? {
        Some(model) => model.id,
        None => return Err(AppError::bad_request("标签不存在")),
    };

    delete_by_id(id, db_conn).await
}

pub async fn list(db_conn: &DatabaseClient) -> AppResult<Vec<Tag>> {
    let mut response = db_conn
        .query("SELECT id, name, created_at, updated_at FROM tag ORDER BY id ASC;")
        .await?;

    response.take(0).map_err(Into::into)
}
