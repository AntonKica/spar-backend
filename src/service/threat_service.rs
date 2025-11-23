use log::error;
use sqlx::{PgConnection, Pool, Postgres};
use crate::model::threat_models::{ThreatCreateModel, ThreatModel};
use crate::service::{next_code_for, ApiError, ApiResult, GeneralService};

pub struct ThreatService;

impl GeneralService<ThreatModel, ThreatModel, ThreatCreateModel> for ThreatService {
    const TABLE_NAME: &'static str = "specific_threat";
    const CODE_PREFIX: &'static str = "THR-";
    const CODE_DIGITS: usize = 10;

    async fn create(tx: &mut PgConnection, create_model: ThreatCreateModel) -> ApiResult<String> {
        let top_code: Option<String> =
            sqlx::query_scalar(r#"SELECT code FROM threat WHERE code LIKE 'THR-%' ORDER BY code DESC LIMIT 1"#)
            .fetch_optional(&mut *tx)
                .await?;
        
        let next_number = match top_code {
            Some(code) => {
                code.strip_prefix(Self::CODE_PREFIX)
                    .ok_or_else(|| {
                        error!("Invalid code format {code}");
                        ApiError::Internal
                    })?
                    .parse::<usize>()
                    .map_err(|e| {
                        error!("Invalid code number {e}");
                        ApiError::Internal
                    })?
                    + 1
            }
            None => 1
        };
        let code = format!("{prefix}{next_number:0number_length$}", prefix=Self::CODE_PREFIX, number_length=Self::CODE_DIGITS - Self::CODE_PREFIX.len());

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