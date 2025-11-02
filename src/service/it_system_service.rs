use sqlx::{Pool, Postgres};

use crate::model::ITSystemModel;
use crate::response::ITSystemResponse;
use crate::service::{ApiError, ApiResult, GeneralService};

pub struct ITSystemService;

impl GeneralService<ITSystemResponse> for ITSystemService {
    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<ITSystemResponse>> {
        let res = sqlx::query_as!(ITSystemModel, r#"SELECT * FROM it_system"#)
            .fetch_all(db)
            .await?;
        Ok(res.into_iter().map(ITSystemResponse::from).collect())
    }

    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<ITSystemResponse> {
        let res = sqlx::query_as!(ITSystemModel, r#"SELECT * FROM it_system WHERE code = $1"#, code)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("ITSystem with code {} not found", code)))?;
        Ok(ITSystemResponse::from(res))
    }
}