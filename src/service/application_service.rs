use sqlx::{Pool, Postgres};

use crate::model::ApplicationModel;
use crate::response::ApplicationResponse;
use crate::service::service::{ApiError, ApiResult, GeneralService};

pub struct ApplicationService;

impl GeneralService<ApplicationResponse> for ApplicationService {
    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<ApplicationResponse>> {
        let res = sqlx::query_as!(ApplicationModel, r#"SELECT * FROM application"#)
            .fetch_all(db)
            .await?;
        Ok(res.into_iter().map(ApplicationResponse::from).collect())
    }

    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<ApplicationResponse> {
        let res = sqlx::query_as!(ApplicationModel, r#"SELECT * FROM application WHERE code = $1"#, code)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("Application with code {} not found", code)))?;
        Ok(ApplicationResponse::from(res))
    }
}