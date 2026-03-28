use crate::{
    AppResponse, AppResponseResult, AppResult, DbConn,
    database::website as db,
    entity::Website,
    model::{Image, UpdateWebsiteInfo},
    service::handler::utils::save_image,
};
use axum::Form;

pub async fn info(DbConn(db_conn): DbConn) -> AppResponseResult<Website> {
    let data = db::info(&db_conn).await?;
    Ok(AppResponse::data(data))
}

pub async fn update_info(
    DbConn(db_conn): DbConn,
    Form(info): Form<UpdateWebsiteInfo>,
) -> AppResult {
    db::update_info(info, &db_conn).await?;
    Ok(())
}

pub async fn update_logo(DbConn(db_conn): DbConn, logo: Image) -> AppResult {
    let path = save_image(logo).await?;
    db::update_logo(&path, &db_conn).await?;
    Ok(())
}

pub async fn update_favicon(DbConn(db_conn): DbConn, favicon: Image) -> AppResult {
    let path = save_image(favicon).await?;
    db::update_favicon(&path, &db_conn).await?;
    Ok(())
}
