use sqlx::{PgConnection, Pool, Postgres};

use crate::model::{BusinessProcessCreateModel, BusinessProcessModel};
use crate::response::BusinessProcessResponse;
use crate::service::{next_code_for, ApiError, ApiResult, GeneralService};

pub struct BusinessProcessService;

impl GeneralService<BusinessProcessResponse, BusinessProcessCreateModel> for BusinessProcessService {
    const TABLE_NAME: &'static str = "business_process";
    const CODE_PREFIX: &'static str = "BP";
    const CODE_DIGITS: usize = 7;

    async fn create(
        tx: &mut PgConnection,
        create_model: BusinessProcessCreateModel,
    ) -> ApiResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;

        sqlx::query!(
        r#"INSERT INTO business_process(code, name, description, process_type) VALUES ($1,$2,$3,$4)"#,
        code,
        create_model.name,
        create_model.description,
        create_model.process_type as i32,
        )
            .execute(tx)
            .await?;

        Ok(code)
    }

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

impl BusinessProcessService {
    pub async fn set_responsible(
        tx: &mut PgConnection,
        business_process_code: String,
        role_code: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"INSERT INTO business_process__role(business_process_code, role_code) VALUES ($1,$2)"#,
        business_process_code,
        role_code,
        )
            .execute(tx)
            .await?;
        Ok(())
    }
    pub async fn assign_role(
        tx: &mut PgConnection,
        business_process_code: String,
        role_code: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"INSERT INTO business_process__role(business_process_code, role_code) VALUES ($1,$2)"#,
        business_process_code,
        role_code,
        )
            .execute(tx)
            .await?;
        Ok(())
    }

    pub async fn assign_application(
        tx: &mut PgConnection,
        business_process_code: String,
        application_code: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"INSERT INTO business_process__application(business_process_code, application_code) VALUES ($1,$2)"#,
        business_process_code,
        application_code,
        )
            .execute(tx)
            .await?;
        Ok(())
    }
}