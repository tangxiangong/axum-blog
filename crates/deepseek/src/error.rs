use reqwest_middleware::Error as HttpError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// 400 格式错误。原因：请求体格式错误
    #[error("请求体格式错误: {0}")]
    BadRequest(String),
    /// 401 认证错误。原因：API KEY 错误，认证失败
    #[error("API KEY 错误，请检查您的 API key 是否正确")]
    Unauthorized,
    /// 402 余额不足。原因：账号余额不足
    #[error("账号余额不足，请充值后重试")]
    PaymentRequired,
    /// 422 参数错误。原因：请求体参数错误
    #[error("请求体参数错误: {0}")]
    ArgumentError(String),
    /// 429 请求速率达到上限。原因：请求速率（TPM 或 RPM）达到上限
    #[error("请求速率达到上限，请稍后重试")]
    TooManyRequests,
    /// 500 服务器故障。原因：服务器内部故障
    #[error("服务器故障，请稍后重试")]
    InternalServerError,
    /// 503 服务器繁忙。原因：服务器负载过高
    #[error("服务器繁忙，请稍后重试")]
    ServiceUnavailable,
    /// TIMEOUT
    #[error("请求超时")]
    Timeout,
    /// 未知错误
    #[error("未知错误: {0}")]
    Unknown(String),
    #[error("内部错误: {0}")]
    Internal(String),
    #[error("序列化错误: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("UTF-8 解码错误: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
    #[error("配置错误: {0}")]
    ConfigError(String),
}

impl From<HttpError> for Error {
    fn from(e: HttpError) -> Self {
        if e.is_timeout() {
            Error::Timeout
        } else if e.is_status() {
            match e.status().unwrap() {
                reqwest::StatusCode::BAD_REQUEST => Error::BadRequest(e.to_string()),
                reqwest::StatusCode::UNAUTHORIZED => Error::Unauthorized,
                reqwest::StatusCode::PAYMENT_REQUIRED => Error::PaymentRequired,
                reqwest::StatusCode::UNPROCESSABLE_ENTITY => Error::ArgumentError(e.to_string()),
                reqwest::StatusCode::TOO_MANY_REQUESTS => Error::TooManyRequests,
                reqwest::StatusCode::INTERNAL_SERVER_ERROR => Error::InternalServerError,
                reqwest::StatusCode::SERVICE_UNAVAILABLE => Error::ServiceUnavailable,
                _ => Error::Unknown(e.to_string()),
            }
        } else {
            Error::Unknown(e.to_string())
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            Error::Timeout
        } else if e.is_status() {
            match e.status().unwrap() {
                reqwest::StatusCode::BAD_REQUEST => Error::BadRequest(e.to_string()),
                reqwest::StatusCode::UNAUTHORIZED => Error::Unauthorized,
                reqwest::StatusCode::PAYMENT_REQUIRED => Error::PaymentRequired,
                reqwest::StatusCode::UNPROCESSABLE_ENTITY => Error::ArgumentError(e.to_string()),
                reqwest::StatusCode::TOO_MANY_REQUESTS => Error::TooManyRequests,
                reqwest::StatusCode::INTERNAL_SERVER_ERROR => Error::InternalServerError,
                reqwest::StatusCode::SERVICE_UNAVAILABLE => Error::ServiceUnavailable,
                _ => Error::Unknown(e.to_string()),
            }
        } else {
            Error::Unknown(e.to_string())
        }
    }
}
