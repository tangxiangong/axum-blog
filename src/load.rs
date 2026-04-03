use crate::{AppState, Setting, SiteInit};
use surrealdb::{engine::any, opt::auth::Root};
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

    let redis_url = format!("redis://{}:{}", setting.redis.host, setting.redis.port);

    let redis_client = redis::Client::open(redis_url).expect("Redis 客户端创建失败");

    let redis = redis::aio::ConnectionManager::new(redis_client)
        .await
        .expect("Redis 连接失败");

    let db = any::connect(setting.db.endpoint.clone())
        .await
        .expect("连接 SurrealDB 失败");

    db.signin(Root {
        username: setting.db.username.clone(),
        password: setting.db.password.clone(),
    })
    .await
    .expect("SurrealDB 登录失败");

    db.use_ns(setting.db.namespace.clone())
        .use_db(setting.db.database.clone())
        .await
        .expect("SurrealDB namespace/database 选择失败");

    let app_state = AppState { db, redis };

    let listner = TcpListener::bind(format!("127.0.0.1:{}", setting.app_port))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to port {}", setting.app_port));

    (listner, app_state, setting.site_init)
}
