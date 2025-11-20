use sqlx::{PgConnection, Pool, Postgres};

use crate::model::asset_model::{AssetCreateModel, AssetDetailModel, AssetModel};
use crate::model::security_measure_models::{SecurityMeasureCreateModel, SecurityMeasureModel};
use crate::service::{next_code_for, ApiError, ApiResult, GeneralService};

pub struct SecurityMeasureService;

impl GeneralService<SecurityMeasureModel, SecurityMeasureModel, SecurityMeasureCreateModel> for SecurityMeasureService {
    const TABLE_NAME: &'static str = "security_measure";
    const CODE_PREFIX: &'static str = "SM";
    const CODE_DIGITS: usize = 10;

    async fn create(
        tx: &mut PgConnection,
        create_model: SecurityMeasureCreateModel,
    ) -> ApiResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;

        sqlx::query!(
        r#"INSERT INTO security_measure VALUES ($1,$2,$3,$4,$5,$6)"#,
        code,
        create_model.name,
        create_model.description,
        create_model.confidentiality_protected,
        create_model.integrity_protected,
        create_model.availability_protected,
        )
            .execute(tx)
            .await?;
        Ok(code)
    }

    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<SecurityMeasureModel>> {
        Ok(sqlx::query_as!(SecurityMeasureModel, r#" SELECT * from security_measure"#).fetch_all(db).await?)
    }

    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<SecurityMeasureModel> {
        let res: SecurityMeasureModel = sqlx::query_as!(SecurityMeasureModel, r#" SELECT * from security_measure"#).fetch_optional(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("Security measure {} not found", code)))?;
        
        
        Ok(res)
    }
}

impl SecurityMeasureService {
    pub async fn list_by_asset_code(db: &Pool<Postgres>, asset_code: String) -> ApiResult<Vec<SecurityMeasureModel>> {
        Ok(sqlx::query_as!(SecurityMeasureModel, r#"
        SELECT sm.* from security_measure sm
        INNER JOIN asset_sm_list asm ON asm.sm_code = sm.code
        WHERE asm.asset_code = $1
         "#, asset_code).fetch_all(db).await?)
    }
}