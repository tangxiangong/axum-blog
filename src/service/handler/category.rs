use crate::{
    AppError, AppResponse, AppResponseResult, AppResult, DbConn,
    database::category as db,
    entity::Category,
    model::{CreateCategory, IdQuery, NameQuery, ParentIdQuery, UpdateCategory},
};
use axum::extract::{Form, Query};

pub async fn find_by_id(
    DbConn(db_conn): DbConn,
    Query(id_query): Query<IdQuery>,
) -> AppResponseResult<Category> {
    let data = db::find_by_id(id_query.id, &db_conn).await?;
    match data {
        Some(data) => Ok(AppResponse::data(data)),
        None => Err(AppError::bad_request("分类不存在")),
    }
}

pub async fn find_by_parent_id(
    DbConn(db_conn): DbConn,
    Query(parent_id_query): Query<ParentIdQuery>,
) -> AppResponseResult<Vec<Category>> {
    let data = db::find_by_parent_id(parent_id_query.parent_id, &db_conn).await?;
    Ok(AppResponse::data(data))
}

pub async fn find_by_name(
    DbConn(db_conn): DbConn,
    Query(name_query): Query<NameQuery>,
) -> AppResponseResult<Category> {
    let data = db::find_by_name(&name_query.name, &db_conn).await?;
    match data {
        Some(data) => Ok(AppResponse::data(data)),
        None => Err(AppError::bad_request("分类不存在")),
    }
}

pub async fn delete_by_id(DbConn(db_conn): DbConn, Query(id_query): Query<IdQuery>) -> AppResult {
    db::delete_by_id(id_query.id, &db_conn).await?;
    Ok(())
}

pub async fn delete_by_name(
    DbConn(db_conn): DbConn,
    Query(name_query): Query<NameQuery>,
) -> AppResult {
    db::delete_by_name(&name_query.name, &db_conn).await?;
    Ok(())
}

pub async fn add(DbConn(db_conn): DbConn, Form(category): Form<CreateCategory>) -> AppResult {
    db::add(category, &db_conn).await?;
    Ok(())
}

pub async fn update(DbConn(db_conn): DbConn, Form(category): Form<UpdateCategory>) -> AppResult {
    db::update(category, &db_conn).await?;
    Ok(())
}

pub async fn list(DbConn(db_conn): DbConn) -> AppResponseResult<Vec<Category>> {
    let data = db::list(&db_conn).await?;
    Ok(AppResponse::data(data))
}
