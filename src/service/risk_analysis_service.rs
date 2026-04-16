use sqlx::{FromRow, PgConnection, Pool, Postgres};
use crate::enums::RiskAnalysisState;
use crate::model::asset_model::AssetModel;
use crate::model::it_grundchutz_models::ItGrundschutzModule;
use crate::service::{ApiError, ApiResult};


#[derive(Debug, Clone, FromRow, serde::Serialize, utoipa::ToSchema)]
pub struct RiskAnalysisModel {
    pub code: String,
    pub state: RiskAnalysisState,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, FromRow, serde::Serialize, utoipa::ToSchema)]
pub struct ModuleWithStatus {
    pub code: String,
    pub name: String,
    pub description: String,
    pub done: bool,
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
                        state AS "state!: RiskAnalysisState"
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

        sqlx::query!(
            r#"
            INSERT INTO module_threat_identification_status (risk_analysis, module)
            SELECT $1::CHAR(6), module
            FROM risk_analysis_module
            WHERE risk_analysis = $1
            "#,
            risk_analysis.code,
        )
            .execute(&mut *tx)
            .await?;

        Ok(risk_analysis.code)
    }

    pub async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<RiskAnalysisModel>> {
        let rows = sqlx::query_as!(
            RiskAnalysisModel,
            r#"
            SELECT
                code,
                created_at AS "created_at!: chrono::DateTime<chrono::Utc>",
                state AS "state!: RiskAnalysisState"
            FROM risk_analysis
            ORDER BY code DESC
            "#
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }

    pub async fn detail(
        db: &Pool<Postgres>,
        code: String,
    ) -> ApiResult<Option<RiskAnalysisModel>> {
        let analysis = sqlx::query_as!(
            RiskAnalysisModel,
            r#"
            SELECT
                code,
                created_at AS "created_at!: chrono::DateTime<chrono::Utc>",
                state AS "state!: RiskAnalysisState"
            FROM risk_analysis
            WHERE code = $1
            "#,
            code,
        )
            .fetch_optional(db)
            .await?;

        Ok(analysis)
    }
    pub async fn list_modules_by_code(
        db: &Pool<Postgres>,
        code: String,
    ) -> ApiResult<Vec<ItGrundschutzModule>> {
        let rows = sqlx::query_as!(
            ItGrundschutzModule,
            r#"
            SELECT m.code, m.name, m.description
            FROM it_grundschutz_module m
            JOIN risk_analysis_module ram ON ram.module = m.code
            WHERE ram.risk_analysis = $1
            ORDER BY m.code
            "#,
            code,
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }

    pub async fn list_assets_by_code(
        db: &Pool<Postgres>,
        code: String,
    ) -> ApiResult<Vec<AssetModel>> {
        let rows = sqlx::query_as!(
            AssetModel,
            r#"
            SELECT a.code, a.name, a.description, a.module
            FROM asset a
            JOIN risk_analysis_asset raa ON raa.asset = a.code
            WHERE raa.risk_analysis = $1
            ORDER BY a.code
            "#,
            code,
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }

    pub async fn list_modules_with_status(
        db: &Pool<Postgres>,
        code: String,
    ) -> ApiResult<Vec<ModuleWithStatus>> {
        let rows = sqlx::query_as!(
            ModuleWithStatus,
            r#"
            SELECT
                m.code,
                m.name,
                m.description,
                mtis.done AS "done!"
            FROM it_grundschutz_module m
            JOIN risk_analysis_module ram ON ram.module = m.code
            JOIN module_threat_identification_status mtis
                ON mtis.risk_analysis = ram.risk_analysis
                AND mtis.module = ram.module
            WHERE ram.risk_analysis = $1
            ORDER BY m.code
            "#,
            code,
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }

    pub async fn set_module_threat_identification_done(
        tx: &mut PgConnection,
        code: String,
        module: String,
    ) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE module_threat_identification_status
            SET done = TRUE
            WHERE risk_analysis = $1 AND module = $2
            "#,
            code,
            module,
        )
            .execute(tx)
            .await?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound(format!(
                "Module {module} not found in risk analysis {code}"
            )));
        }

        Ok(())
    }
}