use crate::enums::{EnumMeta, Risk, RiskAnalysisState};
use crate::enums::{Impact, Likelihood, ThreatCategory};
use crate::model::asset_model::AssetModel;
use crate::model::it_grundchutz_models::{ItGrundschutzModule, ThreatModel};
use crate::service::{ApiError, ApiResult};
use sqlx::{FromRow, PgConnection, Pool, Postgres};

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

#[derive(Debug, Clone, FromRow, serde::Serialize, utoipa::ToSchema)]
pub struct ThreatWithModule {
    pub code: String,
    pub name: String,
    pub description: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
    pub category: ThreatCategory,
    pub module: String,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RiskClassificationModel {
    pub risk_analysis: String,
    pub module: String,
    pub threat: String,
    pub likelihood: Likelihood,
    pub impact: Impact,
    pub evaluation: String,
}

#[derive(Debug, Clone, FromRow, serde::Serialize, utoipa::ToSchema)]
pub struct RiskClassificationDetail {
    pub module: String,
    pub threat_code: String,
    pub threat_name: String,
    pub category: ThreatCategory,
    pub likelihood: Likelihood,
    pub impact: Impact,
    pub evaluation: String,
}

#[derive(Debug, Clone, serde::Deserialize, utoipa::ToSchema)]
pub struct RiskClassificationUpdate {
    pub likelihood: Likelihood,
    pub impact: Impact,
    pub evaluation: String,
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct RiskMatrixCell {
    pub likelihood: Likelihood,
    pub impact: Impact,
    pub risk: Risk,
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct RiskMatrix {
    pub likelihoods: Vec<Likelihood>,
    pub impacts: Vec<Impact>,
    pub cells: Vec<RiskMatrixCell>,
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

        Self::on_enter(&mut *tx, &risk_analysis.code, risk_analysis.state).await?;


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

    pub async fn detail(db: &Pool<Postgres>, code: String) -> ApiResult<Option<RiskAnalysisModel>> {
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

    pub async fn sync_threats(
        tx: &mut PgConnection,
        code: String,
        module: String,
        threats: Vec<String>,
    ) -> ApiResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM risk_analysis_threat
            WHERE risk_analysis = $1 AND module = $2
            "#,
            code,
            module,
        )
        .execute(&mut *tx)
        .await?;

        for threat in &threats {
            sqlx::query!(
                r#"
                INSERT INTO risk_analysis_threat (risk_analysis, module, threat, stage)
                VALUES ($1, $2, $3, 1)
                "#,
                code,
                module,
                threat,
            )
            .execute(&mut *tx)
            .await?;
        }

        Ok(())
    }
    pub async fn list_threats_by_module(
        db: &Pool<Postgres>,
        code: String,
        module: String,
    ) -> ApiResult<Vec<ThreatModel>> {
        let rows = sqlx::query_as!(
            ThreatModel,
            r#"
            SELECT
                t.code,
                t.name,
                t.description,
                t.confidentiality_impaired,
                t.integrity_impaired,
                t.availability_impaired,
                t.category AS "category!: ThreatCategory"
            FROM threat t
            JOIN risk_analysis_threat rat ON rat.threat = t.code
            WHERE rat.risk_analysis = $1 AND rat.module = $2
            ORDER BY t.code
            "#,
            code,
            module,
        )
        .fetch_all(db)
        .await?;
        Ok(rows)
    }
    pub async fn list_all_threats(
        db: &Pool<Postgres>,
        code: String,
    ) -> ApiResult<Vec<ThreatWithModule>> {
        let rows = sqlx::query_as!(
            ThreatWithModule,
            r#"
            SELECT
                t.code,
                t.name,
                t.description,
                t.confidentiality_impaired,
                t.integrity_impaired,
                t.availability_impaired,
                t.category AS "category!: ThreatCategory",
                rat.module
            FROM threat t
            JOIN risk_analysis_threat rat ON rat.threat = t.code
            WHERE rat.risk_analysis = $1
            ORDER BY rat.module, t.code
            "#,
            code,
        )
        .fetch_all(db)
        .await?;

        Ok(rows)
    }

    pub async fn complete_step(
        tx: &mut PgConnection,
        code: String,
        expected_state: RiskAnalysisState,
    ) -> ApiResult<RiskAnalysisState> {
        let current: RiskAnalysisState = sqlx::query_scalar!(
        r#"SELECT state AS "state!: RiskAnalysisState" FROM risk_analysis WHERE code = $1"#,
        code,
    )
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("Risk analysis {code} not found")))?;

        if current != expected_state {
            return Err(ApiError::Validation(format!(
                "Risk analysis {code} is in state {}, expected {}",
                current.code(),
                expected_state.code()
            )));
        }

        let next = current.next().ok_or_else(|| {
            ApiError::Validation(format!("Risk analysis {code} is already at the final step"))
        })?;

        Self::on_enter(&mut *tx, &code, next).await?;

        sqlx::query!(
        r#"UPDATE risk_analysis SET state = $1::risk_analysis_state WHERE code = $2"#,
        next as RiskAnalysisState,
        code,
    )
            .execute(&mut *tx)
            .await?;

        Ok(next)
    }
    async fn on_enter(
        tx: &mut PgConnection,
        code: &str,
        state: RiskAnalysisState,
    ) -> ApiResult<()> {
        match state {
            RiskAnalysisState::ThreatIdentification => {
                sqlx::query!(
                    r#"
            INSERT INTO risk_analysis_asset (risk_analysis, asset)
            SELECT $1::CHAR(6), code FROM asset
            "#,
                    code,
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
                    code,
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
                    code,
                )
                .execute(&mut *tx)
                .await?;

                Ok(())
            }
            RiskAnalysisState::RiskClassification => {
                sqlx::query!(
                    r#"
                    INSERT INTO risk_classification (risk_analysis, module, threat, likelihood, impact)
                    SELECT
                        rat.risk_analysis,
                        rat.module,
                        rat.threat,
                        'rarely'::likelihood,
                        'negligible'::impact
                    FROM risk_analysis_threat rat
                    WHERE rat.risk_analysis = $1
                    "#,
                    code,
                )
                    .execute(tx)
                    .await?;

                Ok(())
            }
        }
    }

    pub fn risk_matrix() -> RiskMatrix {
        let likelihoods = vec![
            Likelihood::Rarely,
            Likelihood::Medium,
            Likelihood::Often,
            Likelihood::VeryOften,
        ];
        let impacts = vec![
            Impact::Negligible,
            Impact::Limited,
            Impact::Significant,
            Impact::LifeThreatening,
        ];

        let cells: Vec<RiskMatrixCell> = impacts
            .iter()
            .flat_map(|&impact| {
                likelihoods.iter().map(move |&likelihood| RiskMatrixCell {
                    likelihood,
                    impact,
                    risk: Risk::from_matrix(likelihood, impact),
                })
            })
            .collect();

        RiskMatrix {
            likelihoods,
            impacts,
            cells,
        }
    }

    pub async fn list_risk_classifications(
        db: &Pool<Postgres>,
        code: String,
    ) -> ApiResult<Vec<RiskClassificationDetail>> {
        let rows = sqlx::query_as!(
            RiskClassificationDetail,
            r#"
            SELECT
                rc.module,
                t.code AS threat_code,
                t.name AS threat_name,
                t.category AS "category!: ThreatCategory",
                rc.likelihood AS "likelihood!: Likelihood",
                rc.impact AS "impact!: Impact",
                rc.evaluation
            FROM risk_classification rc
            JOIN threat t ON t.code = rc.threat
            WHERE rc.risk_analysis = $1
            ORDER BY rc.module, t.code
            "#,
            code,
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }

    pub async fn get_risk_classification(
        db: &Pool<Postgres>,
        code: String,
        module: String,
        threat: String,
    ) -> ApiResult<Option<RiskClassificationDetail>> {
        let row = sqlx::query_as!(
            RiskClassificationDetail,
            r#"
            SELECT
                rc.module,
                t.code AS threat_code,
                t.name AS threat_name,
                t.category AS "category!: ThreatCategory",
                rc.likelihood AS "likelihood!: Likelihood",
                rc.impact AS "impact!: Impact",
                rc.evaluation
            FROM risk_classification rc
            JOIN threat t ON t.code = rc.threat
            WHERE rc.risk_analysis = $1 AND rc.module = $2 AND rc.threat = $3
            "#,
            code,
            module,
            threat,
        )
            .fetch_optional(db)
            .await?;

        Ok(row)
    }

    pub async fn update_risk_classification(
        tx: &mut PgConnection,
        code: String,
        module: String,
        threat: String,
        update: RiskClassificationUpdate,
    ) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE risk_classification
            SET likelihood = $1::likelihood,
                impact = $2::impact,
                evaluation = $3
            WHERE risk_analysis = $4 AND module = $5 AND threat = $6
            "#,
            update.likelihood as Likelihood,
            update.impact as Impact,
            update.evaluation,
            code,
            module,
            threat,
        )
            .execute(tx)
            .await?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound(format!(
                "Risk classification not found for {code}/{module}/{threat}"
            )));
        }

        Ok(())
    }
}
