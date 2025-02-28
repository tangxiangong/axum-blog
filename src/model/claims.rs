use crate::{AppError, AppResult, utils::jwt::*};
use axum::{RequestPartsExt, extract::OptionalFromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Token(pub String);

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Token {
    pub fn payload(&self) -> String {
        get_jwt_payload(&self.0)
    }
}

impl<S> OptionalFromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> AppResult<Option<Self>> {
        match parts
            .extract::<Option<TypedHeader<Authorization<Bearer>>>>()
            .await
        {
            Ok(Some(TypedHeader(Authorization(bearer)))) => {
                Ok(Some(Token(bearer.token().to_string())))
            }
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iat: usize,
    pub exp: usize,
    pub uid: String,
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
        match parts.extract::<Option<Token>>().await {
            Ok(Some(Token(token))) => match Claims::decode(&token) {
                Ok(claims) => Ok(Some(claims)),
                Err(_) => Ok(None),
            },
            _ => Ok(None),
        }
    }
}

impl Claims {
    pub fn exp_secs(&self) -> i64 {
        self.exp as i64 - Local::now().timestamp()
    }

    pub fn new(uid: impl Into<String>) -> Self {
        let iat = Local::now().timestamp() as usize;
        let exp = (Local::now() + Duration::days(15)).timestamp() as usize;
        let uid = uid.into();
        Self { iat, exp, uid }
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
        let token = Claims::new("uuid").encode().unwrap();
        let claims = Claims::decode(&token).unwrap();
        assert_eq!(claims.uid, "uuid");
    }
}
