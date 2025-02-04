use common::{
    AppError, AppResult,
    entity::{ActiveTag, Tag, TagEntity, tag},
};
use sea_orm::{Set, entity::prelude::*};

pub async fn add(tag_name: &str, db_conn: &DbConn) -> AppResult {
    if find_by_name(tag_name, db_conn).await?.is_some() {
        return Err(AppError::bad_request("标签已存在"));
    }
    let _ = ActiveTag {
        name: Set(tag_name.to_owned()),
        ..Default::default()
    }
    .insert(db_conn)
    .await?;
    Ok(())
}

pub async fn find_by_id(id: u32, db_conn: &DbConn) -> AppResult<Option<Tag>> {
    let result = TagEntity::find_by_id(id).one(db_conn).await?;
    Ok(result)
}

pub async fn update(tag_id: u32, new_tag_name: &str, db_conn: &DbConn) -> AppResult {
    if find_by_id(tag_id, db_conn).await?.is_none() {
        return Err(AppError::bad_request("标签不存在"));
    }
    if find_by_name(new_tag_name, db_conn).await?.is_some() {
        return Err(AppError::bad_request("标签已存在"));
    }

    let mut active_model: ActiveTag = TagEntity::find_by_id(tag_id)
        .one(db_conn)
        .await?
        .unwrap()
        .into();

    active_model.name = Set(new_tag_name.to_owned());
    active_model.update(db_conn).await?;
    Ok(())
}

pub async fn find_by_name(name: &str, db_conn: &DbConn) -> AppResult<Option<Tag>> {
    let result = TagEntity::find()
        .filter(tag::Column::Name.eq(name))
        .one(db_conn)
        .await?;
    Ok(result)
}

pub async fn delete_by_id(id: u32, db_conn: &DbConn) -> AppResult {
    if find_by_id(id, db_conn).await?.is_none() {
        return Err(AppError::bad_request("标签不存在"));
    }
    let _ = TagEntity::delete_by_id(id).exec(db_conn).await?;
    Ok(())
}

pub async fn delete_by_name(name: &str, db_conn: &DbConn) -> AppResult {
    let active_model: ActiveTag = match find_by_name(name, db_conn).await? {
        Some(model) => model.into(),
        None => return Err(AppError::bad_request("标签不存在")),
    };
    let _ = active_model.delete(db_conn).await?;
    Ok(())
}

pub async fn list(db_conn: &DbConn) -> AppResult<Vec<Tag>> {
    let result = TagEntity::find().all(db_conn).await?;
    Ok(result)
}
