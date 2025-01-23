use crate::{utils::jwt::*, AppError, AppResult};
use axum::{extract::OptionalFromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Token(pub String);

impl<S> OptionalFromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> AppResult<Option<Self>> {
        if let Ok(Some(TypedHeader(Authorization(bearer)))) = parts
            .extract::<Option<TypedHeader<Authorization<Bearer>>>>()
            .await
        {
            Ok(Some(Token(bearer.token().to_string())))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iat: usize,
    pub exp: usize,
    pub username: String,
}
/// 对 `Claims` 实现 `OptionalFromRequestParts` trait
/// 从请求头 Authorization: Bearer=token 中提取 JWT `token`，并解码为 `Claims`
/// 若因请求头中没有 `token` 而提取失败或者解码 `token` 失败，则返回 `None`
/// 之所以在失败时没有直接返回错误 (`impl FromRequestParts`), 是因为要在认证层中与 Session 机制结合
/// 两个全为 `None` 时，才视作认证失败, 重定向到登录页面
impl<S> OptionalFromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> AppResult<Option<Self>> {
        if let Ok(Some(Token(token))) = parts.extract::<Option<Token>>().await {
            match Claims::decode(&token) {
                Ok(claims) => Ok(Some(claims)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl Claims {
    pub fn exp_secs(&self) -> i64 {
        self.exp as i64 - Local::now().timestamp()
    }

    pub fn new(username: impl Into<String>) -> Self {
        let iat = Local::now().timestamp() as usize;
        let exp = (Local::now() + Duration::days(15)).timestamp() as usize;
        let username = username.into();
        Self { iat, exp, username }
    }

    pub fn encode(&self) -> AppResult<String> {
        let token = encode_jwt(self)?;
        Ok(token)
    }

    pub fn decode(token: &str) -> AppResult<Self> {
        decode_jwt(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt() {
        let token = Claims::new("username").encode().unwrap();
        let claims = Claims::decode(&token).unwrap();
        assert_eq!(claims.username, "username");
    }
}
