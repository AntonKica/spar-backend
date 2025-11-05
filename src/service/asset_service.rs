use sqlx::{PgConnection, Pool, Postgres};

use crate::model::{BusinessProcessCreateModel, AssetCreateModel, AssetModel};
use crate::service::{next_code_for, ApiError, ApiResult, GeneralService};

pub struct AssetService;

impl GeneralService<(), AssetCreateModel> for AssetService {
    const TABLE_NAME: &'static str = "asset";
    const CODE_PREFIX: &'static str = "AST";
    const CODE_DIGITS: usize = 10;

    async fn create(
        tx: &mut PgConnection,
        create_model: AssetCreateModel,
    ) -> ApiResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;
        sqlx::query!(
        r#"INSERT INTO asset(code,
                             name,
                             confidentiality_protection_needs,
                             integrity_protection_needs,
                             availability_protection_needs,
                             description,
                             responsible)

                             VALUES ($1,$2,$3,$4,$5,$6,$7)"#,
        code,
        create_model.name,
        create_model.confidentiality_protection_needs as i32,
        create_model.integrity_protection_needs as i32,
        create_model.availability_protection_needs as i32,
        create_model.description,
            create_model.responsible,
        )
            .execute(tx)
            .await?;
        Ok(code)
    }

    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<()>> {
        todo!()
    }

    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<()> {
        todo!()
    }
}