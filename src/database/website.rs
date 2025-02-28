use crate::{
    AppResult,
    entity::{ActiveWebsite, Website, WebsiteEntity},
    model::UpdateWebsiteInfo,
    set_value,
};
use sea_orm::{Set, entity::prelude::*};

pub async fn info(db_conn: &DbConn) -> AppResult<Website> {
    let model = WebsiteEntity::find().one(db_conn).await?.unwrap();
    Ok(model)
}

pub async fn update_info(info: UpdateWebsiteInfo, db_conn: &DbConn) -> AppResult {
    let mut current_info: ActiveWebsite = WebsiteEntity::find().one(db_conn).await?.unwrap().into();

    set_value!(
        current_info,
        (title, info.title, direct),
        (subtitle, info.subtitle),
        (description, info.description),
    );
    current_info.update(db_conn).await?;
    Ok(())
}

pub async fn update_logo(path: &str, db_conn: &DbConn) -> AppResult {
    let mut current_info: ActiveWebsite = WebsiteEntity::find().one(db_conn).await?.unwrap().into();
    current_info.logo = Set(Some(path.to_owned()));
    current_info.update(db_conn).await?;
    Ok(())
}

pub async fn update_favicon(path: &str, db_conn: &DbConn) -> AppResult {
    let mut current_info: ActiveWebsite = WebsiteEntity::find().one(db_conn).await?.unwrap().into();
    current_info.favicon = Set(Some(path.to_owned()));
    current_info.update(db_conn).await?;
    Ok(())
}
