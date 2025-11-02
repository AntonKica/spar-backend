use sqlx::{PgConnection, Pool, Postgres};

use crate::model::{ITSystemCreateModel, ITSystemModel};
use crate::response::ITSystemResponse;
use crate::service::{next_code_for, ApiError, ApiResult, GeneralService};

pub struct ITSystemService;

impl GeneralService<ITSystemResponse, ITSystemCreateModel> for ITSystemService {
    const TABLE_NAME: &'static str = "it_system";
    const CODE_PREFIX: &'static str = "ITS";
    const CODE_DIGITS: usize = 9;
    async fn create(
        tx: &mut PgConnection,
        create_model: ITSystemCreateModel,
    ) -> ApiResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;
        sqlx::query!(
        r#"INSERT INTO it_system(code, name, description, module_type, count, responsible, application_user) VALUES ($1,$2,$3,$4,$5,$6,$7)"#,
        code,
        create_model.name,
        create_model.description,
        create_model.module_type as i32,
            create_model.count,
            create_model.responsible,
            create_model.application_user
        )
            .execute(tx)
            .await?;
        Ok(code)
    }

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