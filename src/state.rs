use crate::{AppError, AppResult};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;
use surrealdb::{Surreal, engine::any::Any};

pub type DatabaseClient = Surreal<Any>;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseClient,
    pub redis_pool: Pool<RedisConnectionManager>,
}

pub struct DbConn(pub DatabaseClient);

pub type RedisPoolConn = PooledConnection<'static, RedisConnectionManager>;

pub struct RedisConn(pub PooledConnection<'static, RedisConnectionManager>);

impl<S> FromRequestParts<S> for DbConn
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> AppResult<Self> {
        let app_state = AppState::from_ref(state);
        Ok(Self(app_state.db.clone()))
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
