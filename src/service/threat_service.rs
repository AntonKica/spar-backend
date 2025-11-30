use log::error;
use sqlx::{PgConnection, Pool, Postgres};
use crate::model::threat_models::{ThreatCreateModel, ThreatModel};
use crate::service::{next_code_for, next_code_like, ApiError, ApiResult, GeneralService};

pub struct ThreatService;

impl GeneralService<ThreatModel, ThreatModel, ThreatCreateModel> for ThreatService {
    const TABLE_NAME: &'static str = "threat";
    const CODE_PREFIX: &'static str = "THR";
    const CODE_DIGITS: usize = 10;

    async fn create(tx: &mut PgConnection, create_model: ThreatCreateModel) -> ApiResult<String> {
        let code: String = next_code_like(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, &mut *tx).await?;

        sqlx::query(r#"INSERT INTO threat VALUES ($1,$2,$3,$4,$5,$6)"#)
            .bind(code.clone())
            .bind(create_model.name)
            .bind(create_model.description)
            .bind(create_model.confidentiality_impaired)
            .bind(create_model.integrity_impaired)
            .bind(create_model.availability_impaired)
            .execute(&mut *tx)
            .await?;
        
        Ok(code)
    }

    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<ThreatModel>> {
        Ok(sqlx::query_as!(ThreatModel, r#" SELECT * FROM threat"#).fetch_all(db).await?)
    }

    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<ThreatModel> {
        Ok(
            sqlx::query_as!(ThreatModel, r#" SELECT * FROM threat WHERE code = $1 LIMIT 1"#, code)
                .fetch_optional(db)
                .await?
                .ok_or_else(|| ApiError::NotFound(format!("{} not found", code)))?
        )
    }
}