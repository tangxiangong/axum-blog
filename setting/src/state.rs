use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sea_orm::DbConn;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DbConn,
    pub redis_pool: Pool<RedisConnectionManager>,
}
