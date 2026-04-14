use sqlx::{FromRow, PgConnection};
use crate::service::ApiResult;

#[derive(Debug, Clone, Copy, sqlx::Type, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[sqlx(type_name = "risk_analysis_state", rename_all = "snake_case")]
pub enum RiskAnalysisState {
    ThreatIdentification,
}

#[derive(Debug, Clone, FromRow, serde::Serialize, utoipa::ToSchema)]
pub struct RiskAnalysisModel {
    pub code: String,
    pub state: RiskAnalysisState,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct RiskAnalysisService;

impl RiskAnalysisService {
    pub async fn create(tx: &mut PgConnection) -> ApiResult<String> {
        let risk_analysis = sqlx::query_as!(
            RiskAnalysisModel,
            r#"
            INSERT INTO risk_analysis (state)
            VALUES ($1)
            RETURNING   code,
                        created_at AS "created_at!: chrono::DateTime<chrono::Utc>",
                        state
            "#,
            RiskAnalysisState::ThreatIdentification as RiskAnalysisState,
        )
            .fetch_one(&mut *tx)
            .await?;

        sqlx::query!(
            r#"
            INSERT INTO risk_analysis_asset (risk_analysis, asset)
            SELECT $1::CHAR(6), code FROM asset
            "#,
            risk_analysis.code,
        )
            .execute(&mut *tx)
            .await?;

        sqlx::query!(
            r#"
            INSERT INTO risk_analysis_module (risk_analysis, module)
            SELECT DISTINCT $1::CHAR(6), a.module
            FROM asset a
            JOIN risk_analysis_asset ra ON ra.asset = a.code
            WHERE ra.risk_analysis = $1
            "#,
            risk_analysis.code,
        )
            .execute(&mut *tx)
            .await?;

        Ok(risk_analysis.code)
    }
}