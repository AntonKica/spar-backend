use sqlx::{PgConnection, Pool, Postgres};

use crate::model::{BusinessProcessCreateModel, RoleCreateModel, RoleModel};
use crate::response::RoleResponse;
use crate::service::{next_code_for, ApiError, ApiResult, GeneralService};

pub struct RoleService;

impl GeneralService<RoleResponse, RoleCreateModel> for RoleService {
    const TABLE_NAME: &'static str = "role";
    const CODE_PREFIX: &'static str = "RL";
    const CODE_DIGITS: usize = 7;

    async fn create(
        tx: &mut PgConnection,
        create_model: RoleCreateModel,
    ) -> ApiResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;

        sqlx::query!(
        r#"INSERT INTO role(code, name, description) VALUES ($1,$2,$3)"#,
        code,
        create_model.name,
        create_model.description,
        )
            .execute(tx)
            .await?;
        Ok(code)
    }

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