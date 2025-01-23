use crate::{AppState, Setting, SiteInit};
use bb8_redis::RedisConnectionManager;
use std::time::Duration;
use tokio::net::TcpListener;

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

pub async fn get() -> (TcpListener, AppState, SiteInit) {
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

    let redis_url = format!("redis://{}:{}", setting.redis.host, setting.redis.port);

    let manager = RedisConnectionManager::new(redis_url).expect("Redis 连接失败");

    let pool = bb8::Pool::builder()
        .build(manager)
        .await
        .expect("Redis 连接池创建失败");

    let mut opt = sea_orm::ConnectOptions::new(&db_url);
    opt.connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .sqlx_logging(false);

    let db_conn = sea_orm::Database::connect(opt)
        .await
        .expect("连接数据库失败");

    let app_state = AppState {
        db_conn,
        redis_pool: pool,
    };

    let listner = TcpListener::bind(format!("127.0.0.1:{}", setting.app_port))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to port {}", setting.app_port));

    (listner, app_state, setting.site_init)
}
