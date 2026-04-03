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

macro_rules! impl_into_internal_error {
    ($($error:ty),+ $(,)?) => {
        $(
            impl From<$error> for AppError {
            fn from(e: $error) -> Self {
                Self::internal(e.to_string())
                }
            }
        )+
    };
}

macro_rules! impl_into_bad_request_error {
    ($($error:ty),+ $(,)?) => {
        $(
            impl From<$error> for AppError {
            fn from(e: $error) -> Self {
                Self::bad_request(e.to_string())
                }
            }
        )+
    };
}

pub(crate) use impl_from_axum_error;
pub(crate) use impl_into_bad_request_error;
pub(crate) use impl_into_internal_error;
