#[macro_export]
macro_rules! impl_from_axum_error {
    ($($error:ty),+ $(,)?) => {
        $(
            impl From<$error> for AppError {
            fn from(e: $error) -> Self {
                Self::new(e.status(), e.to_string())
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_into_internal_error {
    ($($error:ty),+ $(,)?) => {
        $(
            impl From<$error> for AppError {
            fn from(e: $error) -> Self {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_into_bad_request_error {
    ($($error:ty),+ $(,)?) => {
        $(
            impl From<$error> for AppError {
            fn from(e: $error) -> Self {
                Self::new(StatusCode::BAD_REQUEST, e.to_string())
                }
            }
        )+
    };
}
