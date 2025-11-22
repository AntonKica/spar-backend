use sqlx::{PgConnection, Pool, Postgres};
use crate::model::specific_threat_model::{SpecificThreatCreateModel, SpecificThreatModel};
use crate::service::{next_code_for, ApiError, ApiResult, GeneralService};

pub struct SpecificThreatService;

impl GeneralService<SpecificThreatModel, SpecificThreatModel, SpecificThreatCreateModel> for SpecificThreatService {
    const TABLE_NAME: &'static str = "specific_threat";
    const CODE_PREFIX: &'static str = "THR";
    const CODE_DIGITS: usize = 10;

    async fn create(tx: &mut PgConnection, create_model: SpecificThreatCreateModel) -> ApiResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;

        sqlx::query(r#"INSERT INTO specific_threat VALUES ($1,$2,$3,$4,$5,$6)"#)
            .bind(code.clone())
            .bind(create_model.name)
            .bind(create_model.description)
            .bind(create_model.confidentiality_impaired)
            .bind(create_model.integrity_impaired)
            .bind(create_model.availability_impaired)
            .execute(tx)
            .await?;
        Ok(code)
    }

    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<SpecificThreatModel>> {
        Ok(sqlx::query_as!(SpecificThreatModel, r#" SELECT * FROM specific_threat"#).fetch_all(db).await?)
    }

    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<SpecificThreatModel> {
        Ok(
            sqlx::query_as!(SpecificThreatModel, r#" SELECT * FROM specific_threat WHERE code = $1 LIMIT 1"#, code)
                .fetch_optional(db)
                .await?
                .ok_or_else(|| ApiError::NotFound(format!("{} not found", code)))?
        )
    }
}