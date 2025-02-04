use crate::set_value;
use common::{
    AppError, AppResult,
    entity::{ActiveCategory, Category, CategoryEntity, category},
    model::{CreateCategory, UpdateCategory},
};
use sea_orm::{Set, entity::prelude::*};

pub async fn add(category: CreateCategory, db_conn: &DbConn) -> AppResult {
    if find_by_name(&category.name, db_conn).await?.is_some() {
        return Err(AppError::bad_request("分类已存在"));
    }
    let _ = ActiveCategory {
        name: Set(category.name),
        parent_id: Set(category.parent_id),
        ..Default::default()
    }
    .insert(db_conn)
    .await?;
    Ok(())
}

pub async fn find_by_id(id: u32, db_conn: &DbConn) -> AppResult<Option<Category>> {
    let result = CategoryEntity::find_by_id(id).one(db_conn).await?;
    Ok(result)
}

pub async fn update(category: UpdateCategory, db_conn: &DbConn) -> AppResult {
    if find_by_id(category.id, db_conn).await?.is_none() {
        return Err(AppError::bad_request("分类不存在"));
    }
    if category.name.is_some()
        && find_by_name(&category.name.clone().unwrap(), db_conn)
            .await?
            .is_some()
    {
        return Err(AppError::bad_request("分类已存在"));
    }

    let mut active_model: ActiveCategory = CategoryEntity::find_by_id(category.id)
        .one(db_conn)
        .await?
        .unwrap()
        .into();

    set_value!(
        active_model,
        (name, category.name, direct),
        (parent_id, category.parent_id)
    );
    active_model.update(db_conn).await?;
    Ok(())
}

pub async fn find_by_parent_id(parent_id: u32, db_conn: &DbConn) -> AppResult<Vec<Category>> {
    let result = CategoryEntity::find()
        .filter(category::Column::ParentId.eq(parent_id))
        .all(db_conn)
        .await?;
    Ok(result)
}

pub async fn find_by_name(name: &str, db_conn: &DbConn) -> AppResult<Option<Category>> {
    let result = CategoryEntity::find()
        .filter(category::Column::Name.eq(name))
        .one(db_conn)
        .await?;
    Ok(result)
}

pub async fn delete_by_id(id: u32, db_conn: &DbConn) -> AppResult {
    if find_by_id(id, db_conn).await?.is_none() {
        return Err(AppError::bad_request("分类不存在"));
    }
    let _ = CategoryEntity::delete_by_id(id).exec(db_conn).await?;
    Ok(())
}

pub async fn delete_by_name(name: &str, db_conn: &DbConn) -> AppResult {
    let active_model: ActiveCategory = match find_by_name(name, db_conn).await? {
        Some(model) => model.into(),
        None => return Err(AppError::bad_request("分类不存在")),
    };
    let _ = active_model.delete(db_conn).await?;
    Ok(())
}

pub async fn list(db_conn: &DbConn) -> AppResult<Vec<Category>> {
    let result = CategoryEntity::find().all(db_conn).await?;
    Ok(result)
}
