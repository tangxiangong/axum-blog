use crate::set_value;
use common::{
    AppError, AppResult,
    entity::{ActiveAdmin, Admin, AdminEntity, admin},
    model::UpdateAdminInfo,
    utils::cryption::{decrypt, encrypt},
};
use sea_orm::{DbConn, QuerySelect, Set, prelude::*};

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

pub async fn get_uid(username: &str, db_conn: &DbConn) -> AppResult<String> {
    let uid = AdminEntity::find()
        .select_only()
        .column(admin::Column::Id)
        .filter(admin::Column::Name.eq(username))
        .into_tuple()
        .one(db_conn)
        .await?;
    if let Some(uid) = uid {
        Ok(uid)
    } else {
        Err(AppError::unauth("用户名错误"))
    }
}

pub async fn get_info(uid: &str, db_conn: &DbConn) -> AppResult<Admin> {
    Ok(AdminEntity::find_by_id(uid).one(db_conn).await?.unwrap())
}

pub async fn update_info(uid: &str, info: UpdateAdminInfo, db_conn: &DbConn) -> AppResult<()> {
    let mut admin: ActiveAdmin = AdminEntity::find_by_id(uid)
        .one(db_conn)
        .await?
        .unwrap()
        .into();
    // 原代码替换为
    set_value!(
        admin,
        (name, info.name, direct),
        (nickname, info.nickname),
        (email, info.email),
        (github, info.github),
        (wechat, info.wechat),
        (qq, info.qq)
    );
    admin.update(db_conn).await?;
    Ok(())
}

pub async fn update_avatar(uid: &str, path: &str, db_conn: &DbConn) -> AppResult<()> {
    let mut admin: ActiveAdmin = AdminEntity::find_by_id(uid)
        .one(db_conn)
        .await?
        .unwrap()
        .into();
    admin.avatar = Set(Some(path.to_owned()));
    admin.update(db_conn).await?;
    Ok(())
}

pub async fn update_pwd(uid: &str, pwd: &str, db_conn: &DbConn) -> AppResult {
    let current_pwd = get_password(uid, db_conn).await?;
    if pwd.eq(&current_pwd) {
        return Err(AppError::bad_request("新密码与旧密码相同"));
    }
    let mut admin: ActiveAdmin = AdminEntity::find_by_id(uid)
        .one(db_conn)
        .await?
        .unwrap()
        .into();
    let enc_pwd = encrypt(pwd)?;
    admin.password = Set(enc_pwd);
    admin.update(db_conn).await?;
    Ok(())
}
