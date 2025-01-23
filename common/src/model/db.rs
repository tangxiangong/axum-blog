use crate::{AppError, AppResult};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use bb8::PooledConnection;
use bb8_redis::RedisConnectionManager;
use sea_orm::DbConn;
use setting::AppState;

pub struct MySQLConn(pub DbConn);

pub type RedisPoolConn = PooledConnection<'static, RedisConnectionManager>;

pub struct RedisConn(pub PooledConnection<'static, RedisConnectionManager>);

impl<S> FromRequestParts<S> for MySQLConn
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> AppResult<Self> {
        let app_state = AppState::from_ref(state);
        Ok(Self(app_state.db_conn.clone()))
    }
}

impl<S> FromRequestParts<S> for RedisConn
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> AppResult<Self> {
        let pool = AppState::from_ref(state).redis_pool.clone();
        let conn = pool
            .get_owned()
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(Self(conn))
    }
}
