use sqlx::{PgConnection, Pool, Postgres};
use crate::model::asset_model::AssetCreateModel;
use crate::model::fulfilled_threat_models::{FulfilledThreatCreateModel, FulfilledThreatDetailModel};
use crate::service::{next_code_for, ApiResult};

pub struct FulfilledThreatService;

impl FulfilledThreatService {
    pub async fn create(
        tx: &mut PgConnection,
        create_model: FulfilledThreatCreateModel,
    ) -> ApiResult<String> {
        let code = next_code_for("fulfilled_threat", "FTH", 10, tx).await?;

        sqlx::query_as!(FulfilledThreatCreateModel,
        r#"INSERT INTO fulfilled_threat VALUES ($1,$2,$3,$4,$5,$6)"#,
            code,
            create_model.threat_code,
            create_model.time_cost,
            create_model.time_cost_unit.map(|tcu| tcu as i32),
            create_model.monetary_cost,
            create_model.description,
        )
            .execute(tx)
            .await?;
        Ok(code)
    }

    pub async fn list_detail(db: &Pool<Postgres>) -> ApiResult<Vec<FulfilledThreatDetailModel>> {
        let res = sqlx::query_as!(FulfilledThreatDetailModel,
            r#"
SELECT ft.code,
       threat.code as threat_code,
       threat.name as threat_name,
       ft.time_cost,
       ft.time_cost_unit,
       ft.monetary_cost,
       ft.description,
       threat.confidentiality_impaired,
       threat.integrity_impaired,
       threat.availability_impaired
FROM fulfilled_threat AS ft
         INNER JOIN threat ON ft.threat_code = threat.code
"#)
            .fetch_all(db)
            .await?;

        Ok(res)
    }

    pub async fn list_detail_by_asset_code(db: &Pool<Postgres>, asset_code: String) -> ApiResult<Vec<FulfilledThreatDetailModel>> {
        let res = sqlx::query_as!(FulfilledThreatDetailModel,
            r#"
SELECT ft.code,
       threat.code as threat_code,
       threat.name as threat_name,
       ft.time_cost,
       ft.time_cost_unit,
       ft.monetary_cost,
       ft.description,
       threat.confidentiality_impaired,
       threat.integrity_impaired,
       threat.availability_impaired
FROM fulfilled_threat ft
         INNER JOIN threat ON threat.code = ft.threat_code
         INNER JOIN asset_ft_list aft ON aft.ft_code = ft.code
         WHERE aft.asset_code = $1
"#,asset_code.clone())
            .fetch_all(db)
            .await?;

        Ok(res)
    }
}
