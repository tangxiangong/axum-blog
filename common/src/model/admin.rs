use crate::{AppError, utils::validator::*};
use axum::{
    extract::{Form, FromRequest, FromRequestParts, Query, Request},
    http::request::Parts,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl<S> FromRequest<S> for Login
where
    S: Sync + Send,
{
    type Rejection = AppError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let user = Form::<Login>::from_request(req, _state).await?.0;
        is_valid_username(&user.username)?;
        is_valid_password(&user.password)?;
        Ok(user)
    }
}

#[derive(Debug, Deserialize)]
pub struct RememberMe {
    pub remember_me: bool,
}

impl<S> FromRequestParts<S> for RememberMe
where
    S: Sync + Send,
{
    type Rejection = AppError;
    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match Query::<RememberMe>::from_request_parts(_parts, _state).await {
            Ok(query) => Ok(query.0),
            Err(_) => Ok(RememberMe { remember_me: false }),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateAdminInfo {
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub github: Option<String>,
    pub wechat: Option<String>,
    pub qq: Option<String>,
}

impl<S> FromRequest<S> for UpdateAdminInfo
where
    S: Sync + Send,
{
    type Rejection = AppError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let info = Form::<UpdateAdminInfo>::from_request(req, _state).await?.0;
        if let Some(ref name) = info.name {
            is_valid_username(name)?;
        }

        if let Some(ref email) = info.email {
            is_valid_email(email)?;
        }
        if let Some(ref nickname) = info.nickname {
            is_valid_nickname(nickname)?;
        }
        if let Some(ref url) = info.github {
            is_valid_github(url)?;
        }
        if let Some(ref number) = info.qq {
            is_valid_qq(number)?;
        }
        Ok(info)
    }
}
