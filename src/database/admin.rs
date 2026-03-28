use crate::{
    AppError, AppResult, DatabaseClient,
    database::common::take_one,
    entity::Admin,
    model::UpdateAdminInfo,
    utils::cryption::{decrypt, encrypt},
};
use serde::Deserialize;
use surrealdb::types::SurrealValue;

#[derive(Debug, Deserialize, SurrealValue)]
struct PasswordRow {
    password: Vec<u8>,
}

#[derive(Debug, Deserialize, SurrealValue)]
struct UidRow {
    id: String,
}

pub async fn get_password(name: &str, db_conn: &DatabaseClient) -> AppResult<String> {
    let mut response = db_conn
        .query("SELECT password FROM admin WHERE name = $name LIMIT 1;")
        .bind(("name", name.to_string()))
        .await?;

    let row: Option<PasswordRow> = take_one(&mut response, 0)?;
    match row {
        Some(row) => decrypt(&row.password),
        None => Err(AppError::unauth("用户名错误")),
    }
}

async fn get_password_by_uid(uid: &str, db_conn: &DatabaseClient) -> AppResult<String> {
    let mut response = db_conn
        .query("SELECT password FROM admin WHERE id = $uid LIMIT 1;")
        .bind(("uid", uid.to_string()))
        .await?;

    let row: Option<PasswordRow> = take_one(&mut response, 0)?;
    match row {
        Some(row) => decrypt(&row.password),
        None => Err(AppError::unauth("用户名错误")),
    }
}

pub async fn get_uid(username: &str, db_conn: &DatabaseClient) -> AppResult<String> {
    let mut response = db_conn
        .query("SELECT id FROM admin WHERE name = $username LIMIT 1;")
        .bind(("username", username.to_string()))
        .await?;

    let row: Option<UidRow> = take_one(&mut response, 0)?;
    match row {
        Some(row) => Ok(row.id),
        None => Err(AppError::unauth("用户名错误")),
    }
}

pub async fn get_info(uid: &str, db_conn: &DatabaseClient) -> AppResult<Admin> {
    let mut response = db_conn
        .query(
            "SELECT id, name, password, nickname, email, github, wechat, qq, avatar, created_at, updated_at
             FROM admin
             WHERE id = $uid
             LIMIT 1;",
        )
        .bind(("uid", uid.to_string()))
        .await?;

    take_one(&mut response, 0)?.ok_or_else(|| AppError::bad_request("管理员不存在"))
}

pub async fn update_info(
    uid: &str,
    info: UpdateAdminInfo,
    db_conn: &DatabaseClient,
) -> AppResult<()> {
    let current = get_info(uid, db_conn).await?;

    if let Some(ref new_name) = info.name {
        let mut response = db_conn
            .query("SELECT id FROM admin WHERE name = $name LIMIT 1;")
            .bind(("name", new_name.clone()))
            .await?;
        if let Some(existing) = take_one::<UidRow>(&mut response, 0)?
            && existing.id != uid
        {
            return Err(AppError::bad_request("用户名已存在"));
        }
    }

    let name = info.name.unwrap_or(current.name);
    let nickname = info.nickname.or(current.nickname);
    let email = info.email.or(current.email);
    let github = info.github.or(current.github);
    let wechat = info.wechat.or(current.wechat);
    let qq = info.qq.or(current.qq);

    db_conn
        .query(
            "UPDATE admin
             SET name = $name,
                 nickname = $nickname,
                 email = $email,
                 github = $github,
                 wechat = $wechat,
                 qq = $qq,
                 updated_at = time::now()
             WHERE id = $uid;",
        )
        .bind(("uid", uid.to_string()))
        .bind(("name", name))
        .bind(("nickname", nickname))
        .bind(("email", email))
        .bind(("github", github))
        .bind(("wechat", wechat))
        .bind(("qq", qq))
        .await?;

    Ok(())
}

pub async fn update_avatar(uid: &str, path: &str, db_conn: &DatabaseClient) -> AppResult<()> {
    db_conn
        .query(
            "UPDATE admin
             SET avatar = $avatar,
                 updated_at = time::now()
             WHERE id = $uid;",
        )
        .bind(("uid", uid.to_string()))
        .bind(("avatar", Some(path.to_owned())))
        .await?;

    Ok(())
}

pub async fn update_pwd(uid: &str, pwd: &str, db_conn: &DatabaseClient) -> AppResult {
    let current_pwd = get_password_by_uid(uid, db_conn).await?;
    if pwd.eq(&current_pwd) {
        return Err(AppError::bad_request("新密码与旧密码相同"));
    }

    let enc_pwd = encrypt(pwd)?;
    db_conn
        .query(
            "UPDATE admin
             SET password = $password,
                 updated_at = time::now()
             WHERE id = $uid;",
        )
        .bind(("uid", uid.to_string()))
        .bind(("password", enc_pwd))
        .await?;

    Ok(())
}
