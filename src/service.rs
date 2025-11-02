use crate::model::BusinessProcessModel;
use crate::response::BusinessProcessResponse;
use actix_web::{HttpResponse, ResponseError};
use sqlx::{Pool, Postgres};
use thiserror::Error;

pub mod business_process_service;
pub mod role_service;
pub mod application_service;
pub mod it_system_service;
pub mod risk_analysis_process_service;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal server error")]
    Internal,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::Database(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "message": "Database error occurred"
                }))
            }
            ApiError::NotFound(msg) => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "status": "error",
                    "message": msg
                }))
            }
            ApiError::Validation(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "status": "error",
                    "message": msg
                }))
            }
            ApiError::Internal => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "message": "Internal server error"
                }))
            }
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

pub trait GeneralService<T> {
    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<T>>;
    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<T>;
}