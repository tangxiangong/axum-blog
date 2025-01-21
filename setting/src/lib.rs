use sea_orm::prelude::*;
use serde::Deserialize;
use std::time::Duration;
use tokio::net::TcpListener;

#[derive(Deserialize, Clone)]
pub struct Setting {
    pub app_port: u16,
    pub db: DBSetting,
    #[serde(rename = "site")]
    pub site_init: SiteInit,
}

impl Default for Setting {
    fn default() -> Self {
        Setting {
            app_port: 3000,
            db: DBSetting::default(),
            site_init: SiteInit::default(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct DBSetting {
    #[serde(rename = "type")]
    pub db_type: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub username: String,
    pub password: String,
}

impl Default for DBSetting {
    fn default() -> Self {
        DBSetting {
            db_type: "mysql".to_string(),
            host: "localhost".to_string(),
            port: 3306,
            db_name: "blog".to_string(),
            username: "root".to_string(),
            password: "password".to_string(),
        }
    }
}

fn load() -> Setting {
    match config::Config::builder()
        .add_source(config::File::with_name("configration"))
        .build()
    {
        Ok(config) => config.try_deserialize::<Setting>().unwrap_or_else(|_| {
            tracing::info!("配置文件解析失败，使用默认配置");
            Setting::default()
        }),
        Err(_) => {
            tracing::info!("配置文件读取失败，使用默认配置");
            Setting::default()
        }
    }
}

pub async fn get() -> (DbConn, TcpListener, SiteInit) {
    let setting = load();

    let db_url = format!(
        "{}://{}:{}@{}:{}/{}",
        setting.db.db_type,
        setting.db.username,
        setting.db.password,
        setting.db.host,
        setting.db.port,
        setting.db.db_name
    );

    let mut opt = sea_orm::ConnectOptions::new(&db_url);
    opt.connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .sqlx_logging(false);

    let db_conn = sea_orm::Database::connect(opt)
        .await
        .expect("连接数据库失败");

    let listner = TcpListener::bind(format!("127.0.0.1:{}", setting.app_port))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to port {}", setting.app_port));

    (db_conn, listner, setting.site_init)
}

#[derive(Deserialize, Clone)]
pub struct SiteInit {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    #[serde(rename = "admin")]
    pub admin_init: AdminInit,
}

impl Default for SiteInit {
    fn default() -> Self {
        SiteInit {
            title: "Blog with Axum".to_string(),
            subtitle: "".to_string(),
            description: "".to_string(),
            admin_init: AdminInit {
                name: "admin".to_string(),
                password: "password".to_string(),
            },
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct AdminInit {
    pub name: String,
    pub password: String,
}
