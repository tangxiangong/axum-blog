use axum::{
    extract::{Form, FromRequest, FromRequestParts, Query, Request},
    http::request::Parts,
};
use serde::Deserialize;

use crate::{
    utils::validator::{is_valid_password, is_valid_username},
    AppError,
};

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
pub struct LoginQuery {
    pub remember_me: bool,
}

impl<S> FromRequestParts<S> for LoginQuery
where
    S: Sync + Send,
{
    type Rejection = AppError;
    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match Query::<LoginQuery>::from_request_parts(_parts, _state).await {
            Ok(query) => Ok(query.0),
            Err(_) => Ok(LoginQuery { remember_me: false }),
        }
    }
}
