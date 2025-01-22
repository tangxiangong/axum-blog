use crate::{impl_from_axum_error, impl_into_internal_error, AppResponse};
use axum::{
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AppError {
    #[serde(skip)]
    status_code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: String) -> Self {
        Self {
            status_code: code,
            message,
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        let message = message.into();
        Self::new(StatusCode::FORBIDDEN, message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        let message = message.into();
        Self::new(StatusCode::CONFLICT, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        let message = message.into();
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn unauth(message: impl Into<String>) -> Self {
        let message = message.into();
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        let message = message.into();
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn code(&self) -> StatusCode {
        self.status_code
    }
}

impl_from_axum_error!(
    axum::extract::rejection::FormRejection,
    axum::extract::rejection::JsonRejection,
    axum::extract::rejection::PathRejection,
    axum::extract::rejection::QueryRejection,
);

impl_into_internal_error!(
    std::io::Error,
    sea_orm::error::DbErr,
    sea_orm::error::SqlErr,
    serde_json::Error,
);

impl From<AppError> for AppResponse {
    fn from(value: AppError) -> Self {
        AppResponse::error(value.status_code, value.message)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let code = self.status_code;
        let mut res = AppResponse::from(self).into_response();
        if let StatusCode::UNAUTHORIZED = code {
            let bearer = HeaderValue::from_str("Bearer").unwrap();
            res.headers_mut().insert("WWW-Authenticate", bearer);
            res
        } else {
            res
        }
    }
}

pub type AppResult<T = ()> = Result<T, AppError>;

pub type AppResponseResult<T = (), M = ()> = AppResult<AppResponse<T, M>>;
