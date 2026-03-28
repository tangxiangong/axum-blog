use crate::AppState;
use crate::{Setting, SiteInit};
use bb8_redis::RedisConnectionManager;
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

    let manager = RedisConnectionManager::new(redis_url).expect("Redis 连接失败");

    let pool = bb8::Pool::builder()
        .build(manager)
        .await
        .expect("Redis 连接池创建失败");

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

    let app_state = AppState {
        db,
        redis_pool: pool,
    };

    let listner = TcpListener::bind(format!("127.0.0.1:{}", setting.app_port))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to port {}", setting.app_port));

    (listner, app_state, setting.site_init)
}
