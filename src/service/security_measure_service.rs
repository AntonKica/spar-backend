use sqlx::{FromRow, PgConnection, Pool, Postgres};
use crate::enums::RiskTreatmentType;
use crate::service::ApiResult;

#[derive(Debug, Clone, FromRow, serde::Serialize, utoipa::ToSchema)]
pub struct SecurityMeasure {
    pub code: String,
    pub treatment: RiskTreatmentType,
    pub description: String,
}

#[derive(Debug, Clone, serde::Deserialize, utoipa::ToSchema)]
pub struct SecurityMeasureCreate {
    pub treatment: RiskTreatmentType,
    pub description: String,
}
pub struct SecurityMeasureService;

impl SecurityMeasureService {
    pub async fn create(
        tx: &mut PgConnection,
        model_create: SecurityMeasureCreate,
    ) -> ApiResult<String> {
        let seq = match model_create.treatment {
            RiskTreatmentType::Avoid => "security_measure_avd_seq",
            RiskTreatmentType::Reduce => "security_measure_red_seq",
            RiskTreatmentType::Transfer => "security_measure_tsf_seq",
            RiskTreatmentType::Accept => "security_measure_acp_seq",
        };

        let prefix = match model_create.treatment {
            RiskTreatmentType::Avoid => "AVD-",
            RiskTreatmentType::Reduce => "RED-",
            RiskTreatmentType::Transfer => "TSF-",
            RiskTreatmentType::Accept => "ACP-",
        };

        let query = format!(
            "INSERT INTO security_measure (code, treatment, description) VALUES ('{prefix}' || LPAD(nextval('{seq}')::TEXT, 4, '0'), $1::risk_treatment_type, $2) RETURNING code"
        );

        let rec = sqlx::query_scalar::<_, String>(query.as_str())
            .bind(model_create.treatment)
            .bind(&model_create.description)
            .fetch_one(tx)
            .await?;

        Ok(rec)
    }

    pub async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<SecurityMeasure>> {
        let rows = sqlx::query_as!(
            SecurityMeasure,
            r#"
            SELECT
                code,
                treatment AS "treatment!: RiskTreatmentType",
                description
            FROM security_measure
            ORDER BY code
            "#
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }

    pub async fn list_by_treatment(
        db: &Pool<Postgres>,
        treatment: RiskTreatmentType,
    ) -> ApiResult<Vec<SecurityMeasure>> {
        let rows = sqlx::query_as!(
            SecurityMeasure,
            r#"
            SELECT
                code,
                treatment AS "treatment!: RiskTreatmentType",
                description
            FROM security_measure
            WHERE treatment = $1
            ORDER BY code
            "#,
            treatment as RiskTreatmentType,
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }
    
}