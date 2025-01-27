use super::utils;
use axum::Extension;
use common::{
    entity::Admin,
    model::{Image, UpdateAdminInfo},
    AppResponse, AppResponseResult, AppResult, MySQLConn,
};
use database::admin as db;

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
