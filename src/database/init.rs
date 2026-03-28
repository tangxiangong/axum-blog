use crate::{AppResult, DatabaseClient, SiteInit, utils::cryption::encrypt};
use serde::Deserialize;
use surrealdb::types::SurrealValue;

#[derive(Debug, Deserialize, SurrealValue)]
struct AdminExists {
    id: String,
}

#[derive(Debug, Deserialize, SurrealValue)]
struct WebsiteExists {
    title: String,
}

async fn init_schema(db_conn: &DatabaseClient) -> AppResult {
    db_conn
        .query(
            r#"
DEFINE TABLE IF NOT EXISTS counter SCHEMALESS;
DEFINE TABLE IF NOT EXISTS admin SCHEMALESS;
DEFINE TABLE IF NOT EXISTS jwt SCHEMALESS;
DEFINE TABLE IF NOT EXISTS category SCHEMALESS;
DEFINE TABLE IF NOT EXISTS tag SCHEMALESS;
DEFINE TABLE IF NOT EXISTS article SCHEMALESS;
DEFINE TABLE IF NOT EXISTS comment SCHEMALESS;
DEFINE TABLE IF NOT EXISTS article_category SCHEMALESS;
DEFINE TABLE IF NOT EXISTS article_tag SCHEMALESS;
DEFINE TABLE IF NOT EXISTS website SCHEMALESS;

DEFINE INDEX IF NOT EXISTS idx_admin_name ON TABLE admin FIELDS name UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_jwt_token ON TABLE jwt FIELDS token UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_category_name ON TABLE category FIELDS name UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_tag_name ON TABLE tag FIELDS name UNIQUE;
"#,
        )
        .await?;
    Ok(())
}

async fn init_admin(site: &SiteInit, db_conn: &DatabaseClient) -> AppResult {
    let mut response = db_conn.query("SELECT id FROM admin LIMIT 1;").await?;
    let admins: Vec<AdminExists> = response.take(0)?;
    if !admins.is_empty() {
        return Ok(());
    }

    let enc_pwd = encrypt(&site.admin_init.password)?;
    let id = uuid::Uuid::new_v4().to_string();

    db_conn
        .query(
            "CREATE type::thing('admin', $rid) CONTENT {
                id: $id,
                name: $name,
                password: $password,
                nickname: NONE,
                email: NONE,
                github: NONE,
                wechat: NONE,
                qq: NONE,
                avatar: NONE,
                created_at: time::now(),
                updated_at: time::now()
            };",
        )
        .bind(("rid", id.clone()))
        .bind(("id", id))
        .bind(("name", site.admin_init.name.clone()))
        .bind(("password", enc_pwd))
        .await?;

    Ok(())
}

async fn init_website(site: &SiteInit, db_conn: &DatabaseClient) -> AppResult {
    let mut response = db_conn.query("SELECT title FROM website LIMIT 1;").await?;
    let websites: Vec<WebsiteExists> = response.take(0)?;
    if !websites.is_empty() {
        return Ok(());
    }

    db_conn
        .query(
            "CREATE type::thing('website', 'main') CONTENT {
                title: $title,
                subtitle: $subtitle,
                description: $description,
                logo: NONE,
                favicon: NONE,
                created_at: time::now(),
                updated_at: time::now()
            };",
        )
        .bind(("title", site.title.clone()))
        .bind(("subtitle", Some(site.subtitle.clone())))
        .bind(("description", Some(site.description.clone())))
        .await?;

    Ok(())
}

pub async fn init(site: SiteInit, db_conn: &DatabaseClient) -> AppResult {
    init_schema(db_conn).await?;
    init_admin(&site, db_conn).await?;
    init_website(&site, db_conn).await?;
    Ok(())
}
