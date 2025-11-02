use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: &'static str,
    pub data: T,
}

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub status: &'static str,
    pub message: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            status: "ok",
            data,
        }
    }
}

impl ApiErrorResponse {
    pub fn new(message: String) -> Self {
        Self {
            status: "error",
            message,
        }
    }
}
pub fn to_error_response<E: std::fmt::Display>(error: E) -> ApiErrorResponse {
    ApiErrorResponse::new(error.to_string())
}