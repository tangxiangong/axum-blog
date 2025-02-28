use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct AppResponse<T = (), M = ()> {
    #[serde(skip)]
    status_code: StatusCode,
    code: u16,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<M>,
}

impl<T, M> AppResponse<T, M> {
    pub fn new(data: T, meta: M) -> Self {
        let status_code = StatusCode::OK;
        Self {
            status_code,
            code: status_code.as_u16(),
            status: status_code
                .canonical_reason()
                .unwrap_or("Unknown")
                .to_string(),
            message: None,
            data: Some(data),
            meta: Some(meta),
        }
    }

    pub fn created() -> Self {
        Self {
            status_code: StatusCode::CREATED,
            code: StatusCode::CREATED.as_u16(),
            status: StatusCode::CREATED
                .canonical_reason()
                .unwrap_or("Unknown")
                .to_string(),
            message: None,
            data: None,
            meta: None,
        }
    }

    pub fn data(data: T) -> Self {
        let status_code = StatusCode::OK;
        Self {
            status_code,
            code: status_code.as_u16(),
            status: status_code
                .canonical_reason()
                .unwrap_or("Unknown")
                .to_string(),
            message: None,
            data: Some(data),
            meta: None,
        }
    }
}

impl AppResponse {
    pub fn ok() -> Self {
        Self {
            status_code: StatusCode::OK,
            code: StatusCode::OK.as_u16(),
            status: StatusCode::OK
                .canonical_reason()
                .unwrap_or("Unknown")
                .to_string(),
            message: None,
            data: None,
            meta: None,
        }
    }

    pub fn error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code,
            code: status_code.as_u16(),
            status: status_code
                .canonical_reason()
                .unwrap_or("Unknown")
                .to_string(),
            message: Some(message),
            data: None,
            meta: None,
        }
    }
}

impl<T: Serialize, M: Serialize> IntoResponse for AppResponse<T, M> {
    fn into_response(self) -> Response {
        (self.status_code, Json(json!(self))).into_response()
    }
}
