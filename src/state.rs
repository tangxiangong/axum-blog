use crate::{AppError, AppResult};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use surrealdb::{Surreal, engine::any::Any};

pub type DatabaseClient = Surreal<Any>;
pub type RedisClient = redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseClient,
    pub redis: RedisClient,
}

pub struct DbConn(pub DatabaseClient);

pub struct RedisConn(pub RedisClient);

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
        Ok(Self(AppState::from_ref(state).redis.clone()))
    }
}
