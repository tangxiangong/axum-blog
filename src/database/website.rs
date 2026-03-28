use crate::{
    AppError, AppResult, DatabaseClient, database::common::take_one, entity::Website,
    model::UpdateWebsiteInfo,
};

pub async fn info(db_conn: &DatabaseClient) -> AppResult<Website> {
    let mut response = db_conn
        .query(
            "SELECT title, subtitle, description, logo, favicon, created_at, updated_at
             FROM website
             LIMIT 1;",
        )
        .await?;

    take_one(&mut response, 0)?.ok_or_else(|| AppError::bad_request("网站信息不存在"))
}

pub async fn update_info(info: UpdateWebsiteInfo, db_conn: &DatabaseClient) -> AppResult {
    let current = self::info(db_conn).await?;
    let title = info.title.unwrap_or(current.title.clone());
    let subtitle = info.subtitle.or(current.subtitle);
    let description = info.description.or(current.description);

    db_conn
        .query(
            "UPDATE website
             SET title = $title,
                 subtitle = $subtitle,
                 description = $description,
                 updated_at = time::now()
             WHERE title = $current_title;",
        )
        .bind(("current_title", current.title))
        .bind(("title", title))
        .bind(("subtitle", subtitle))
        .bind(("description", description))
        .await?;

    Ok(())
}

pub async fn update_logo(path: &str, db_conn: &DatabaseClient) -> AppResult {
    let current = self::info(db_conn).await?;
    db_conn
        .query(
            "UPDATE website
             SET logo = $logo,
                 updated_at = time::now()
             WHERE title = $current_title;",
        )
        .bind(("current_title", current.title))
        .bind(("logo", Some(path.to_owned())))
        .await?;

    Ok(())
}

pub async fn update_favicon(path: &str, db_conn: &DatabaseClient) -> AppResult {
    let current = self::info(db_conn).await?;
    db_conn
        .query(
            "UPDATE website
             SET favicon = $favicon,
                 updated_at = time::now()
             WHERE title = $current_title;",
        )
        .bind(("current_title", current.title))
        .bind(("favicon", Some(path.to_owned())))
        .await?;

    Ok(())
}
