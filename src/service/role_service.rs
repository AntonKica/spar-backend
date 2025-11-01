use sqlx::{Pool, Postgres};

use crate::model::RoleModel;
use crate::response::RoleResponse;
use crate::service::service::{ApiError, ApiResult, GeneralService};

pub struct RoleService;

impl GeneralService<RoleResponse> for RoleService {
    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<RoleResponse>> {
        let res = sqlx::query_as!(RoleModel, r#"SELECT * FROM role"#)
            .fetch_all(db)
            .await?;
        Ok(res.into_iter().map(RoleResponse::from).collect())
    }

    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<RoleResponse> {
        let res = sqlx::query_as!(RoleModel, r#"SELECT * FROM role WHERE code = $1"#, code)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("Role with code {} not found", code)))?;
        Ok(RoleResponse::from(res))
    }
}

impl RoleService {
    pub async fn list_for_business_process(db: &Pool<Postgres>, business_process_code: String) -> ApiResult<Vec<RoleResponse>> {
        let res = sqlx::query_as!(RoleModel,
    r#"SELECT * FROM role WHERE EXISTS(
        SELECT * FROM business_process__role WHERE business_process__role.role_code = role.code AND business_process__role.business_process_code = $1 LIMIT 1)"#
            ,business_process_code
        )
            .fetch_all(db)
            .await?;
        Ok(res.into_iter().map(RoleResponse::from).collect())
    }
}