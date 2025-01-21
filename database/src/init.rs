use common::{
    entity::{ActiveAdmin, ActiveWebsite, AdminEntity, WebsiteEntity},
    utils::cryption::encrypt,
    AppResult,
};
use sea_orm::{prelude::*, DbConn, Set};
use setting::SiteInit;

pub async fn init(site: SiteInit, db_conn: &DbConn) -> AppResult {
    if AdminEntity::find().one(db_conn).await?.is_none() {
        let en_pwd = encrypt(&site.admin_init.password)?;
        ActiveAdmin {
            name: Set(site.admin_init.name),
            password: Set(en_pwd),
            ..Default::default()
        }
        .insert(db_conn)
        .await
        .expect("管理员账号初始化失败");
    }
    if WebsiteEntity::find().one(db_conn).await?.is_none() {
        ActiveWebsite {
            title: Set(site.title),
            subtitle: Set(Some(site.subtitle)),
            description: Set(Some(site.description)),
            ..Default::default()
        }
        .insert(db_conn)
        .await
        .expect("网站信息初始化失败");
    }
    Ok(())
}
