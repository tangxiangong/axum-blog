use crate::{
    AppResponse, AppResponseResult, AppResult, MySQLConn,
    database::website as db,
    entity::Website,
    model::{Image, UpdateWebsiteInfo},
    service::handler::utils::save_image,
};
use axum::Form;

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
