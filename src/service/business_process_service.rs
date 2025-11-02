use sqlx::{Pool, Postgres};

use crate::model::BusinessProcessModel;
use crate::response::BusinessProcessResponse;
use crate::service::{ApiError, ApiResult, GeneralService};

pub struct BusinessProcessService;

impl GeneralService<BusinessProcessResponse> for BusinessProcessService {
    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<BusinessProcessResponse>> {
        let res = sqlx::query_as!(BusinessProcessModel, r#"SELECT * FROM business_process"#)
            .fetch_all(db)
            .await?;
        Ok(res.into_iter().map(BusinessProcessResponse::from).collect())
    }

    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<BusinessProcessResponse> {
        let res = sqlx::query_as!(BusinessProcessModel, r#"SELECT * FROM business_process WHERE code = $1"#, code)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("BusinessProcess with code {} not found", code)))?;
        Ok(BusinessProcessResponse::from(res))
    }
}