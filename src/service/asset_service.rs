use sqlx::{PgConnection, Pool, Postgres};

use crate::model::asset_model::{AssetCreateModel, AssetDetailModel, AssetModel};
use crate::service::{next_code_for, ApiError, ApiResult, GeneralService};
use crate::service::fulfilled_threat_service::FulfilledThreatService;
use crate::service::security_measure_service::SecurityMeasureService;

pub struct AssetService;

impl GeneralService<AssetModel, AssetDetailModel, AssetCreateModel> for AssetService {
    const TABLE_NAME: &'static str = "asset";
    const CODE_PREFIX: &'static str = "AST";
    const CODE_DIGITS: usize = 10;

    async fn create(
        tx: &mut PgConnection,
        create_model: AssetCreateModel,
    ) -> ApiResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;

        sqlx::query!(
        r#"INSERT INTO asset VALUES ($1,$2,$3,$4,$5,$6,$7)"#,
            code,
            create_model.name,
            create_model.asset_type as i32,
            create_model.confidentiality_protection_needs as i32,
            create_model.integrity_protection_needs as i32,
            create_model.availability_protection_needs as i32,
            create_model.description,
        )
            .execute(tx)
            .await?;
        Ok(code)
    }

    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<AssetModel>> {
        Ok(sqlx::query_as!(AssetModel, r#" SELECT * FROM asset"#).fetch_all(db).await?)
    }
    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<AssetDetailModel> {
        let asset: AssetModel = sqlx::query_as!(AssetModel, r#"SELECT * FROM asset WHERE code = $1"#, code.clone()).fetch_optional(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("Asset {} not found", code)))?;

        let fulfilled_threat_list = FulfilledThreatService::list_detail_by_asset_code(&db, code.clone()).await?;
        let security_measure_list = SecurityMeasureService::list_by_asset_code(&db, code).await?;

        Ok(AssetDetailModel{
                code: asset.code,
                name: asset.name,
                asset_type: asset.asset_type,
                confidentiality_protection_needs: asset.confidentiality_protection_needs,
                integrity_protection_needs: asset.integrity_protection_needs,
                availability_protection_needs: asset.availability_protection_needs,
                description: asset.description,
                fulfilled_threat_list,
                security_measure_list,
            })
    }
}

impl AssetService {
    pub async fn list_for_risk_analysis_process(db: &Pool<Postgres>, rap_code: String) -> ApiResult<Vec<AssetModel>> {
        Ok(
            sqlx::query_as!(AssetModel,
                r#"
                SELECT * FROM asset
                 WHERE EXISTS(SELECT * FROM rap_tour_list
                     WHERE asset.code = rap_tour_list.asset_code
                     AND rap_tour_list.rap_code = $1
                     LIMIT 1
                 )
                "#, rap_code)
                .fetch_all(db)
                .await?
        )
    }

    pub async fn assign_security_measure(
        tx: &mut PgConnection,
        asset_code: String,
        sm_code: String,
    ) -> ApiResult<()> {
        sqlx::query!(r#"INSERT INTO asset_sm_list VALUES ($1, $2)"#, asset_code, sm_code).execute(&mut *tx).await?;

        Ok(())
    }

    pub async fn assign_fulfilled_threat(
        tx: &mut PgConnection,
        asset_code: String,
        ft_code: String,
    ) -> ApiResult<()> {
        sqlx::query!(r#"INSERT INTO asset_ft_list VALUES ($1, $2)"#, asset_code, ft_code).execute(&mut *tx).await?;

        Ok(())
    }
}