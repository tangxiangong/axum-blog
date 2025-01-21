use common::{
    entity::{admin, AdminEntity},
    utils::cryption::decrypt,
    AppError, AppResult,
};
use sea_orm::{prelude::*, DbConn, QuerySelect};

pub async fn get_password(name: &str, db_conn: &DbConn) -> AppResult<String> {
    let en_password = AdminEntity::find()
        .select_only()
        .column(admin::Column::Password)
        .filter(admin::Column::Name.eq(name))
        .into_tuple::<Vec<u8>>()
        .one(db_conn)
        .await?;
    if let Some(en_password) = en_password {
        let password = decrypt(&en_password)?;
        Ok(password)
    } else {
        Err(AppError::unauth("用户名错误"))
    }
}
