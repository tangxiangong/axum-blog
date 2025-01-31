use reqwest::Error as HttpError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum DeepSeekError {
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
}

impl From<HttpError> for DeepSeekError {
    fn from(e: HttpError) -> Self {
        if e.is_timeout() {
            DeepSeekError::Timeout
        } else if e.is_status() {
            match e.status().unwrap() {
                reqwest::StatusCode::BAD_REQUEST => DeepSeekError::BadRequest(e.to_string()),
                reqwest::StatusCode::UNAUTHORIZED => DeepSeekError::Unauthorized,
                reqwest::StatusCode::PAYMENT_REQUIRED => DeepSeekError::PaymentRequired,
                reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                    DeepSeekError::ArgumentError(e.to_string())
                }
                reqwest::StatusCode::TOO_MANY_REQUESTS => DeepSeekError::TooManyRequests,
                reqwest::StatusCode::INTERNAL_SERVER_ERROR => DeepSeekError::InternalServerError,
                reqwest::StatusCode::SERVICE_UNAVAILABLE => DeepSeekError::ServiceUnavailable,
                _ => DeepSeekError::Unknown(e.to_string()),
            }
        } else {
            DeepSeekError::Unknown(e.to_string())
        }
    }
}
