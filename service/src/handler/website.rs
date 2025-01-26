use axum::Form;
use common::{
    entity::Website,
    model::{Image, UpdateWebsiteInfo},
    AppResponse, AppResponseResult, AppResult, MySQLConn,
};
use database::website as db;

use crate::handler::utils::save_image;

pub async fn info(MySQLConn(db_conn): MySQLConn) -> AppResponseResult<Website> {
    let data = db::info(&db_conn).await?;
    Ok(AppResponse::data(data))
}

pub async fn update_info(
    MySQLConn(db_conn): MySQLConn,
    Form(info): Form<UpdateWebsiteInfo>,
) -> AppResult {
    db::update_info(info, &db_conn).await?;
    Ok(())
}

pub async fn update_logo(MySQLConn(db_conn): MySQLConn, logo: Image) -> AppResult {
    let path = save_image(logo).await?;
    db::update_logo(&path, &db_conn).await?;
    Ok(())
}

pub async fn update_favicon(MySQLConn(db_conn): MySQLConn, favicon: Image) -> AppResult {
    let path = save_image(favicon).await?;
    db::update_logo(&path, &db_conn).await?;
    Ok(())
}
