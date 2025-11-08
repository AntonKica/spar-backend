use actix_web::{HttpResponse, ResponseError};
use log::error;
use sqlx::{PgConnection, Pool, Postgres};
use thiserror::Error;

pub mod business_process_service;
pub mod role_service;
pub mod application_service;
pub mod it_system_service;
pub mod risk_analysis_process_service;
pub mod asset_service;
pub mod risk_classification_service;

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
            ApiError::Database(e) => {
                println!("{e}");
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

pub async fn next_code_for_db(
    table: &str,
    acronym: &str,
    code_length: usize,
    db: &Pool<Postgres>
) -> ApiResult<String> {
    // TOTAL LENGTH - STRING - '-'
    let prefix = format!("{acronym}-");
    let number_length = code_length - prefix.len();

    let query = format!("SELECT code FROM {table} ORDER BY code DESC LIMIT 1");
    let top_code: Option<String> = sqlx::query_scalar(&query).fetch_optional(db).await?;
    let next_number = match top_code {
        Some(code) => {
            code.strip_prefix(&prefix)
                .ok_or_else(|| {
                    error!("Invalid code format {}", code.to_owned());
                    ApiError::Internal
                })?
                .parse::<usize>()
                .map_err(|e| {
                    error!("Invalid code number {e}");
                    ApiError::Internal
                })?
                + 1
        }
        None => 1
    };
    Ok(format!("{prefix}{next_number:0number_length$}"))
}

    pub async fn next_code_for(
    table: &str,
    acronym: &str,
    code_length: usize,
    tx: &mut PgConnection
) -> ApiResult<String> {
    // TOTAL LENGTH - STRING - '-'
    let prefix = format!("{acronym}-");
    let number_length = code_length - prefix.len();

    let query = format!("SELECT code FROM {table} ORDER BY code DESC LIMIT 1");
    let top_code: Option<String> = sqlx::query_scalar(&query).fetch_optional(tx).await?;
    let next_number = match top_code {
        Some(code) => {
            code.strip_prefix(&prefix)
                .ok_or_else(|| {
                    error!("Invalid code format {}", code.to_owned());
                    ApiError::Internal
                })?
                .parse::<usize>()
                .map_err(|e| {
                    error!("Invalid code number {e}");
                    ApiError::Internal
                })?
                + 1
        }
        None => 1
    };

    Ok(format!("{prefix}{next_number:0number_length$}"))
}
pub trait GeneralService<T, U> {
    const TABLE_NAME: &'static str;
    const CODE_PREFIX: &'static str;
    const CODE_DIGITS: usize;

    async fn create(tx: &mut PgConnection, create_model: U) -> ApiResult<String>;
    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<T>>;
    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<T>;
}