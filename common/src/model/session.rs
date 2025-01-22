use crate::{AppError, AppResult};
use axum::{extract::OptionalFromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{headers::Cookie, TypedHeader};
use chrono::{DateTime, Duration, Local};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Expiry {
    OnSessionEnd,
    OnInactivity(Duration),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    id: String,
    data: HashMap<String, Value>,
    expiry: Expiry,
    last_accessed: DateTime<Local>,
}

impl Default for Session {
    fn default() -> Self {
        let id = Uuid::new_v4().to_string();
        let data = HashMap::new();
        let expiry = Expiry::OnSessionEnd;
        let last_accessed = Local::now();
        Self {
            id,
            data,
            expiry,
            last_accessed,
        }
    }
}

impl Session {
    pub fn new(expiry: Expiry) -> Self {
        Self {
            expiry,
            ..Default::default()
        }
    }

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

    pub fn update(&mut self) {
        self.last_accessed = Local::now();
    }

    pub fn is_expired(&self) -> bool {
        if let Expiry::OnInactivity(duration) = self.expiry {
            Local::now() - self.last_accessed > duration
        } else {
            false
        }
    }

    pub fn save(&self) {
        todo!()
    }

    pub fn load(_id: &str) -> AppResult<Option<Self>> {
        todo!()
    }
}

impl<S> OptionalFromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> AppResult<Option<Self>> {
        if let Ok(Some(TypedHeader(cookies))) = parts.extract::<Option<TypedHeader<Cookie>>>().await
        {
            match cookies.get("JSESSIONID") {
                Some(id) => {
                    if let Ok(Some(session)) = Self::load(id) {
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
