use sqlx::{PgConnection, Pool, Postgres};
use crate::model::specific_threat_model::{SpecificThreatCreateModel, SpecificThreatModel};
use crate::service::{next_code_for, ApiResult, GeneralService};

pub struct SpecificThreatService;

impl GeneralService<SpecificThreatModel, SpecificThreatModel, SpecificThreatCreateModel> for SpecificThreatService {
    const TABLE_NAME: &'static str = "THR";
    const CODE_PREFIX: &'static str = "";
    const CODE_DIGITS: usize = 10;

    async fn create(tx: &mut PgConnection, create_model: SpecificThreatCreateModel) -> ApiResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;

        sqlx::query!(
        r#"INSERT INTO specific_threat VALUES ($1,$2,$3,$4,$5,$6)"#,
            code,
            create_model.name,
            create_model.description,
            create_model.confidentiality_impaired,
            create_model.integrity_impaired,
            create_model.availability_impaired
        )
            .execute(tx)
            .await?;

        Ok(code)
    }

    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<SpecificThreatModel>> {
        todo!()
    }

    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<SpecificThreatModel> {
        todo!()
    }
}