use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use std::time::Instant;

/// 计算响应耗时的中间件
/// 会在响应头中添加 `X-Response-Elapsed` 字段
pub async fn timing(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let mut res = next.run(req).await;
    let elapsed = start.elapsed().as_millis();
    let headermap = res.headers_mut();
    match HeaderValue::from_str(&format!("{}ms", elapsed)) {
        Ok(v) => {
            headermap.insert("X-Response-Elapsed", v);
            res
        }
        _ => res,
    }
}
