use sqlx::{PgConnection, Pool, Postgres};

use crate::model::{ApplicationCreateModel, ApplicationModel, RoleCreateModel};
use crate::response::ApplicationResponse;
use crate::service::{next_code_for, ApiError, ApiResult, GeneralService};

pub struct ApplicationService;

impl GeneralService<ApplicationResponse, ApplicationCreateModel> for ApplicationService {
    const TABLE_NAME: &'static str = "application";
    const CODE_PREFIX: &'static str = "APP";
    const CODE_DIGITS: usize = 9;

    async fn create(
        tx: &mut PgConnection,
        create_model: ApplicationCreateModel,
    ) -> ApiResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;

        sqlx::query!(
        r#"INSERT INTO application(code, name, description, module_type, responsible, application_user) VALUES ($1,$2,$3,$4,$5,$6)"#,
        code,
        create_model.name,
        create_model.description,
        create_model.module_type as i32,
            create_model.responsible,
            create_model.application_user
        )
            .execute(tx)
            .await?;
        Ok(code)
    }
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