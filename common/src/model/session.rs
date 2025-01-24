use crate::{model::RedisPoolConn, AppError, AppResult};
use axum::{
    extract::{FromRef, OptionalFromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{headers::Cookie, TypedHeader};
use chrono::{DateTime, Duration, Local};
use redis::AsyncCommands;
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use setting::AppState;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Expiry {
    OnSessionEnd,
    OnInactivity(Duration),
}

#[derive(Debug, Serialize, Deserialize, FromRedisValue, ToRedisArgs, Eq, PartialEq)]
pub struct Session {
    id: String,
    data: HashMap<String, Value>,
    expiry: Expiry,
    // jwt_payload: Option<String>,
    last_accessed: DateTime<Local>,
}

impl Default for Session {
    fn default() -> Self {
        let id = Uuid::new_v4().to_string();
        let data = HashMap::new();
        let expiry = Expiry::OnSessionEnd;
        let last_accessed = Local::now();
        // let jwt_payload = None;
        Self {
            id,
            data,
            expiry,
            // jwt_payload,
            last_accessed,
        }
    }
}

impl Session {
    const DEFAULT_EXP: Duration = Duration::hours(12);

    pub fn new(expiry: Expiry) -> Self {
        Self {
            expiry,
            ..Default::default()
        }
    }

    // pub fn with_payload(payload: Option<String>) -> Self {
    //     Self {
    //         jwt_payload: payload,
    //         ..Default::default()
    //     }
    // }

    // pub fn payload(&self) -> Option<String> {
    //     self.jwt_payload.clone()
    // }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn insert(&mut self, key: &str, value: impl Serialize) -> AppResult {
        let key = key.into();
        let value = serde_json::to_value(value)?;
        self.data.insert(key, value);
        Ok(())
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> AppResult<Option<T>> {
        let value = self.data.get(key);
        if let Some(v) = value {
            Ok(serde_json::from_value(v.clone())?)
        } else {
            Ok(None)
        }
    }

    pub async fn update(&mut self, conn: RedisPoolConn) -> AppResult {
        self.last_accessed = Local::now();
        self.save(conn).await?;
        Ok(())
    }

    pub fn is_expired(&self) -> bool {
        if let Expiry::OnInactivity(duration) = self.expiry {
            Local::now() - self.last_accessed > duration
        } else {
            false
        }
    }

    pub async fn save(&self, mut conn: RedisPoolConn) -> AppResult {
        let key = self.id();
        if conn.exists(key).await? {
            let _: () = conn.del(key).await?;
        }

        let _: () = conn.set(key, self).await?;
        let exp = match self.expiry {
            Expiry::OnSessionEnd => Self::DEFAULT_EXP.num_seconds(),
            Expiry::OnInactivity(duration) => duration.num_seconds(),
        };
        let _: () = conn.expire(key, exp).await?;

        Ok(())
    }

    pub async fn load(id: &str, mut conn: RedisPoolConn) -> AppResult<Option<Self>> {
        if conn.exists(id).await? {
            let session: Session = conn.get(id).await?;
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }
}

impl<S> OptionalFromRequestParts<S> for Session
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> AppResult<Option<Self>> {
        let pool = AppState::from_ref(state).redis_pool.clone();
        let conn = pool
            .get_owned()
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if let Ok(Some(TypedHeader(cookies))) = parts.extract::<Option<TypedHeader<Cookie>>>().await
        {
            match cookies.get("SESSION_ID") {
                Some(id) => {
                    if let Ok(Some(session)) = Self::load(id, conn).await {
                        Ok(Some(session))
                    } else {
                        Ok(None)
                    }
                }
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
