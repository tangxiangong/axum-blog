use axum::Extension;
use common::AppResponse;

pub async fn test(Extension(uid): Extension<String>) -> AppResponse<String> {
    AppResponse::<String>::data(uid)
}
