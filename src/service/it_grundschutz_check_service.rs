use sqlx::{PgConnection, Pool, Postgres};
use crate::service::risk_analysis_service::{RiskAssessmentUpdateModel, RiskTreatmentModel};
use crate::enums::{ImplementationStatus, RiskTreatmentType};
use crate::service::{ApiError, ApiResult};

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct AssessmentItem {
    pub id: uuid::Uuid,
    pub code: String,
    pub description: String,
    pub status: ImplementationStatus,
    pub evaluation: String,
    pub kind: String, // "requirement" or "security_measure"
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct OrgAssessment {
    pub treatment_type: RiskTreatmentType,
    pub items: Vec<AssessmentItem>,
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct ThreatAssessment {
    pub threat_code: String,
    pub threat_name: String,
    pub treatment_type: RiskTreatmentType,
    pub items: Vec<AssessmentItem>,
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct ModuleAssessment {
    pub module_code: String,
    pub module_name: String,
    pub treatment_type: RiskTreatmentType,
    pub items: Vec<AssessmentItem>,
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct ModuleThreatAssessment {
    pub module_code: String,
    pub threat_code: String,
    pub threat_name: String,
    pub treatment_type: RiskTreatmentType,
    pub items: Vec<AssessmentItem>,
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct FullAssessment {
    pub org: Option<OrgAssessment>,
    pub threats: Vec<ThreatAssessment>,
    pub modules: Vec<ModuleAssessment>,
    pub module_threats: Vec<ModuleThreatAssessment>,
}

pub struct ItGrundschutCheckService;
impl ItGrundschutCheckService {
    pub async fn full_assessment(
        conn: impl sqlx::Acquire<'_, Database = sqlx::Postgres>,
        code: String,
    ) -> ApiResult<FullAssessment> {
        let mut conn = conn.acquire().await?;
        let treatments = sqlx::query_as!(
            RiskTreatmentModel,
            r#"
            SELECT
                id,
                risk_analysis,
                module,
                threat,
                treatment AS "treatment!: RiskTreatmentType",
                description
            FROM risk_treatment
            WHERE risk_analysis = $1
            ORDER BY module, threat
            "#,
            code,
        )
            .fetch_all(&mut *conn)
            .await?;

        let req_assessments = sqlx::query!(
            r#"
            SELECT
                a.id,
                a.risk_treatment,
                r.code,
                r.description,
                a.status AS "status!: ImplementationStatus",
                a.evaluation
            FROM risk_treatment_requirement_assessment a
            JOIN it_grundschutz_module_requirement r ON r.code = a.requirement
            WHERE a.risk_analysis = $1
            ORDER BY r.code
            "#,
            code,
        )
            .fetch_all(&mut *conn)
            .await?;

        let sm_assessments = sqlx::query!(
            r#"
            SELECT
                a.id,
                a.risk_treatment,
                sm.code,
                sm.description,
                a.status AS "status!: ImplementationStatus",
                a.evaluation
            FROM risk_treatment_security_measure_assessment a
            JOIN security_measure sm ON sm.code = a.security_measure
            WHERE a.risk_analysis = $1
            ORDER BY sm.code
            "#,
            code,
        )
            .fetch_all(&mut *conn)
            .await?;

        let threat_names: std::collections::HashMap<String, String> = sqlx::query!(
            r#"
            SELECT code, name FROM threat
            "#
        )
            .fetch_all(&mut *conn)
            .await?
            .into_iter()
            .map(|r| (r.code, r.name))
            .collect();

        let module_names: std::collections::HashMap<String, String> = sqlx::query!(
            r#"
            SELECT code, name FROM it_grundschutz_module
            "#
        )
            .fetch_all(&mut *conn)
            .await?
            .into_iter()
            .map(|r| (r.code, r.name))
            .collect();

        let mut org: Option<OrgAssessment> = None;
        let mut threats: Vec<ThreatAssessment> = Vec::new();
        let mut modules: Vec<ModuleAssessment> = Vec::new();
        let mut module_threats: Vec<ModuleThreatAssessment> = Vec::new();

        for treatment in &treatments {
            let mut items: Vec<AssessmentItem> = Vec::new();

            for r in &req_assessments {
                if r.risk_treatment == treatment.id {
                    items.push(AssessmentItem {
                        id: r.id,
                        code: r.code.clone(),
                        description: r.description.clone(),
                        status: r.status,
                        evaluation: r.evaluation.clone(),
                        kind: "requirement".to_string(),
                    });
                }
            }

            for s in &sm_assessments {
                if s.risk_treatment == treatment.id {
                    items.push(AssessmentItem {
                        id: s.id,
                        code: s.code.clone(),
                        description: s.description.clone(),
                        status: s.status,
                        evaluation: s.evaluation.clone(),
                        kind: "security_measure".to_string(),
                    });
                }
            }

            match (&treatment.module, &treatment.threat) {
                (None, None) => {
                    org = Some(OrgAssessment {
                        treatment_type: treatment.treatment,
                        items,
                    });
                }
                (None, Some(threat)) => {
                    threats.push(ThreatAssessment {
                        threat_code: threat.clone(),
                        threat_name: threat_names.get(threat).cloned().unwrap_or_default(),
                        treatment_type: treatment.treatment,
                        items,
                    });
                }
                (Some(module), None) => {
                    modules.push(ModuleAssessment {
                        module_code: module.clone(),
                        module_name: module_names.get(module).cloned().unwrap_or_default(),
                        treatment_type: treatment.treatment,
                        items,
                    });
                }
                (Some(module), Some(threat)) => {
                    module_threats.push(ModuleThreatAssessment {
                        module_code: module.clone(),
                        threat_code: threat.clone(),
                        threat_name: threat_names.get(threat).cloned().unwrap_or_default(),
                        treatment_type: treatment.treatment,
                        items,
                    });
                }
            }
        }

        Ok(FullAssessment {
            org,
            threats,
            modules,
            module_threats,
        })
    }

    pub async fn update_security_measure_assessment(
        tx: &mut PgConnection,
        id: uuid::Uuid,
        update: RiskAssessmentUpdateModel,
    ) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE risk_treatment_security_measure_assessment
            SET status = $1::implementation_status,
                evaluation = $2
            WHERE id = $3
            "#,
            update.status as ImplementationStatus,
            update.evaluation,
            id,
        )
            .execute(tx)
            .await?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound(format!(
                "Security measure assessment {id} not found"
            )));
        }

        Ok(())
    }

    pub async fn update_requirement_assessment(
        tx: &mut PgConnection,
        id: uuid::Uuid,
        update: RiskAssessmentUpdateModel,
    ) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE risk_treatment_requirement_assessment
            SET status = $1::implementation_status,
                evaluation = $2
            WHERE id = $3
            "#,
            update.status as ImplementationStatus,
            update.evaluation,
            id,
        )
            .execute(tx)
            .await?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound(format!(
                "Requirement assessment {id} not found"
            )));
        }

        Ok(())
    }
    pub async fn is_risk_treatment_assessment_completed(
        db: &Pool<Postgres>,
        code: String,
    ) -> ApiResult<bool> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT
                (SELECT COUNT(*) FROM risk_treatment_requirement_assessment WHERE risk_analysis = $1 AND status = 'not_assessed'::implementation_status)
                +
                (SELECT COUNT(*) FROM risk_treatment_security_measure_assessment WHERE risk_analysis = $1 AND status = 'not_assessed'::implementation_status)
                AS "count!"
            "#,
            code,
        )
            .fetch_one(db)
            .await?;

        Ok(count == 0)
    }
}