use crate::{utils::jwt::*, AppError, AppResult};
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iat: usize,
    pub exp: usize,
    pub username: String,
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> AppResult<Self> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::unauth("Bearer 授权头缺失"))?;
        let claims = Claims::decode(bearer.token())?;

        Ok(claims)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthInfo {
    #[serde(rename = "type")]
    pub token_type: String,
    pub token: String,
}

impl AuthInfo {
    pub fn new(token: impl Into<String>) -> Self {
        let token = token.into();
        Self {
            token_type: "Bearer".to_string(),
            token,
        }
    }
}

impl Claims {
    pub fn new(username: impl Into<String>) -> Self {
        let iat = Local::now().timestamp() as usize;
        let exp = (Local::now() + Duration::days(15)).timestamp() as usize;
        let username = username.into();
        Self { iat, exp, username }
    }

    pub fn encode(&self) -> AppResult<AuthInfo> {
        let token = encode_jwt(self)?;
        Ok(AuthInfo::new(token))
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
        let auth_info = Claims::new("username").encode().unwrap();
        println!("{:?}", auth_info);
        let claims = Claims::decode(&auth_info.token).unwrap();
        assert_eq!(claims.username, "username");
    }
}
