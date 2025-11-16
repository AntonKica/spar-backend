use serde::Serialize;
use serde_json::Value;

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

#[derive(Serialize)]
pub struct ApiCodeResponse {
    pub status: &'static str,
    pub code: String,
}

#[derive(Serialize)]
pub struct ApiSuccessResponse {
    pub status: &'static str,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            status: "ok",
            data,
        }
    }
}

impl ApiCodeResponse {
    pub fn new(code: String) -> Self {
        Self {
            status: "ok",
            code,
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
impl ApiSuccessResponse {
    pub fn new() -> Self {
        Self {
            status: "ok",
        }
    }
}
pub fn to_error_response<E: std::fmt::Display>(error: E) -> ApiErrorResponse {
    ApiErrorResponse::new(error.to_string())
}