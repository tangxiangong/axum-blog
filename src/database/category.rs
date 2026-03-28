use crate::{
    AppError, AppResult, DatabaseClient,
    database::common::{next_id, take_one},
    entity::Category,
    model::{CreateCategory, UpdateCategory},
};

pub async fn add(category: CreateCategory, db_conn: &DatabaseClient) -> AppResult {
    if find_by_name(&category.name, db_conn).await?.is_some() {
        return Err(AppError::bad_request("分类已存在"));
    }

    let id = next_id("category", db_conn).await?;
    db_conn
        .query(
            "CREATE type::thing('category', $rid) CONTENT {
                id: $id,
                name: $name,
                parent_id: $parent_id,
                created_at: time::now(),
                updated_at: time::now()
            };",
        )
        .bind(("rid", id))
        .bind(("id", id))
        .bind(("name", category.name))
        .bind(("parent_id", category.parent_id))
        .await?;

    Ok(())
}

pub async fn find_by_id(id: u32, db_conn: &DatabaseClient) -> AppResult<Option<Category>> {
    let mut response = db_conn
        .query(
            "SELECT id, name, parent_id, created_at, updated_at
             FROM category
             WHERE id = $id
             LIMIT 1;",
        )
        .bind(("id", id))
        .await?;

    take_one(&mut response, 0)
}

pub async fn update(category: UpdateCategory, db_conn: &DatabaseClient) -> AppResult {
    let current = match find_by_id(category.id, db_conn).await? {
        Some(current) => current,
        None => return Err(AppError::bad_request("分类不存在")),
    };

    if let Some(ref new_name) = category.name
        && let Some(existing) = find_by_name(new_name, db_conn).await?
        && existing.id != category.id
    {
        return Err(AppError::bad_request("分类已存在"));
    }

    let name = category.name.unwrap_or(current.name);
    let parent_id = category.parent_id.or(current.parent_id);

    db_conn
        .query(
            "UPDATE category
             SET name = $name,
                 parent_id = $parent_id,
                 updated_at = time::now()
             WHERE id = $id;",
        )
        .bind(("id", category.id))
        .bind(("name", name))
        .bind(("parent_id", parent_id))
        .await?;

    Ok(())
}

pub async fn find_by_parent_id(
    parent_id: u32,
    db_conn: &DatabaseClient,
) -> AppResult<Vec<Category>> {
    let mut response = db_conn
        .query(
            "SELECT id, name, parent_id, created_at, updated_at
             FROM category
             WHERE parent_id = $parent_id
             ORDER BY id ASC;",
        )
        .bind(("parent_id", parent_id))
        .await?;

    response.take(0).map_err(Into::into)
}

pub async fn find_by_name(name: &str, db_conn: &DatabaseClient) -> AppResult<Option<Category>> {
    let mut response = db_conn
        .query(
            "SELECT id, name, parent_id, created_at, updated_at
             FROM category
             WHERE name = $name
             LIMIT 1;",
        )
        .bind(("name", name.to_string()))
        .await?;

    take_one(&mut response, 0)
}

pub async fn delete_by_id(id: u32, db_conn: &DatabaseClient) -> AppResult {
    if find_by_id(id, db_conn).await?.is_none() {
        return Err(AppError::bad_request("分类不存在"));
    }

    db_conn
        .query("DELETE category WHERE id = $id;")
        .bind(("id", id))
        .await?;
    Ok(())
}

pub async fn delete_by_name(name: &str, db_conn: &DatabaseClient) -> AppResult {
    let id = match find_by_name(name, db_conn).await? {
        Some(model) => model.id,
        None => return Err(AppError::bad_request("分类不存在")),
    };

    delete_by_id(id, db_conn).await
}

pub async fn list(db_conn: &DatabaseClient) -> AppResult<Vec<Category>> {
    let mut response = db_conn
        .query("SELECT id, name, parent_id, created_at, updated_at FROM category ORDER BY id ASC;")
        .await?;

    response.take(0).map_err(Into::into)
}
