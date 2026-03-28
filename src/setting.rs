use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Setting {
    pub app_port: u16,
    pub db: DBSetting,
    pub redis: RedisSetting,
    #[serde(rename = "site")]
    pub site_init: SiteInit,
}

impl Default for Setting {
    fn default() -> Self {
        Setting {
            app_port: 3000,
            redis: RedisSetting::default(),
            db: DBSetting::default(),
            site_init: SiteInit::default(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct RedisSetting {
    pub host: String,
    pub port: u16,
}

impl Default for RedisSetting {
    fn default() -> Self {
        RedisSetting {
            host: "localhost".to_string(),
            port: 6379,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct DBSetting {
    pub endpoint: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl Default for DBSetting {
    fn default() -> Self {
        DBSetting {
            endpoint: "ws://localhost:8000/rpc".to_string(),
            namespace: "blog".to_string(),
            database: "blog".to_string(),
            username: "root".to_string(),
            password: "password".to_string(),
        }
    }
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
