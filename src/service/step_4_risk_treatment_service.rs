use crate::service::{next_code_like, ApiError, ApiResult};
use sqlx::{PgConnection, Pool, Postgres};
use crate::enums::step_3_risk_classification_enums::ThreatRisk;
use crate::enums::step_4_risk_treatment_enums::RiskTreatmentType;
use crate::model::asset_model::AssetModel;
use crate::model::step_3_risk_classification_models::TourRiskClassificationModel;
use crate::model::step_4_risk_treatment_models::{RiskAcceptanceCreateModel, RiskAcceptanceModel, RiskAvoidanceCreateModel, RiskAvoidanceModel, RiskReductionCreateModel, RiskReductionModel, RiskTransferCreateModel, RiskTransferModel, RiskTreatmentModel, TourRiskClassificationCalculatedModel};
use crate::service::step_3_risk_classification_service::Step3RiskClassificationService;

pub struct Step4RiskTreatmentService;

struct TourRiskClassificationCalculatedRow {
    threat_code: String,
    threat_name: String,
    confidentiality_impaired: bool,
    integrity_impaired: bool,
    availability_impaired: bool,

    probability: i32,
    impact: i32,
    treatment_type: Option<i32>,
}

impl Step4RiskTreatmentService {
    pub async fn initialize_step(
        tx: &mut PgConnection,
        rap_code: String,
    ) -> ApiResult<()> {
        Ok(())
    }
    pub async fn tour_risk_classification_list(
        db: &Pool<Postgres>,
        rap_code: String,
        tour_code: String,
    ) -> ApiResult<Vec<TourRiskClassificationCalculatedModel>> {

        // treatment_type is Option yet SQLX treat it as an i32
        let risk_classification_list = sqlx::query_as_unchecked!(TourRiskClassificationCalculatedRow, r#"
            SELECT
                threat.code as threat_code,
                threat.name as threat_name,
                confidentiality_impaired,
                availability_impaired,
                integrity_impaired,
                probability,
                impact,
                treatment_type
            FROM risk_classification
            LEFT OUTER JOIN risk_treatment ON
                risk_treatment.rap_code = risk_classification.rap_code AND
                risk_treatment.tour_code = risk_classification.tour_code AND
                risk_treatment.threat_code = risk_classification.threat_code
            INNER JOIN threat ON threat.code = risk_classification.threat_code
            WHERE risk_classification.rap_code = $1 AND risk_classification.tour_code = $2
            "#, rap_code, tour_code)
            .fetch_all(db)
            .await?;

        Ok(
            risk_classification_list.into_iter().map(|rc| {
                TourRiskClassificationCalculatedModel {
                    threat_code: rc.threat_code,
                    threat_name: rc.threat_name,
                    confidentiality_impaired: rc.confidentiality_impaired,
                    integrity_impaired: rc.integrity_impaired,
                    availability_impaired: rc.availability_impaired,
                    risk: Step3RiskClassificationService::risk_matrix(rc.probability, rc.impact) as i32,
                    treatment_type: rc.treatment_type
                }}
            ).collect()
        )
    }


    pub async fn get_risk_acceptance(
        db: &Pool<Postgres>,
        rap_code: String,
        tour_code: String,
        threat_code: String,
    ) -> ApiResult<Option<RiskAcceptanceModel>> {
        Ok(
            sqlx::query_as!(RiskAcceptanceModel, r#"
            SELECT * FROM risk_acceptance
            WHERE EXISTS(
                SELECT *
                FROM risk_treatment
                WHERE rap_code = $1 AND tour_code = $2 AND threat_code = $3 AND risk_treatment.treatment_code = risk_acceptance.code
            )"#, rap_code, tour_code, threat_code)
                .fetch_optional(db)
                .await?
        )
    }

    pub async fn list_risk_acceptance(db: &Pool<Postgres>) -> ApiResult<Vec<RiskAcceptanceModel>> {
        Ok(sqlx::query_as!(RiskAcceptanceModel, r#" SELECT * FROM risk_acceptance"#).fetch_all(db).await?)
    }

    pub async fn get_risk_avoidance(
        db: &Pool<Postgres>,
        rap_code: String,
        tour_code: String,
        threat_code: String,
    ) -> ApiResult<Option<RiskAvoidanceModel>> {
        Ok(
            sqlx::query_as!(RiskAvoidanceModel, r#"
            SELECT * FROM risk_avoidance
            WHERE EXISTS(
                SELECT *
                FROM risk_treatment
                WHERE rap_code = $1 AND tour_code = $2 AND threat_code = $3 AND risk_treatment.treatment_code = risk_avoidance.code
            )"#, rap_code, tour_code, threat_code)
                .fetch_optional(db)
                .await?
        )
    }

    pub async fn list_risk_avoidance(db: &Pool<Postgres>) -> ApiResult<Vec<RiskAvoidanceModel>> {
        Ok(sqlx::query_as!(RiskAvoidanceModel, r#" SELECT * FROM risk_avoidance"#).fetch_all(db).await?)
    }

    pub async fn get_risk_transfer(
        db: &Pool<Postgres>,
        rap_code: String,
        tour_code: String,
        threat_code: String,
    ) -> ApiResult<Option<RiskTransferModel>> {
        Ok(
            sqlx::query_as!(RiskTransferModel, r#"
            SELECT * FROM risk_transfer
            WHERE EXISTS(
                SELECT *
                FROM risk_treatment
                WHERE rap_code = $1 AND tour_code = $2 AND threat_code = $3 AND risk_treatment.treatment_code = risk_transfer.code
            )"#, rap_code, tour_code, threat_code)
                .fetch_optional(db)
                .await?
        )
    }

    pub async fn list_risk_reduction(db: &Pool<Postgres>) -> ApiResult<Vec<RiskReductionModel>> {
        Ok(sqlx::query_as!(RiskReductionModel, r#" SELECT * FROM risk_reduction"#).fetch_all(db).await?)
    }

    pub async fn get_risk_reduction(
        db: &Pool<Postgres>,
        rap_code: String,
        tour_code: String,
        threat_code: String,
    ) -> ApiResult<Vec<RiskReductionModel>> {
        Ok(
            sqlx::query_as!(RiskReductionModel, r#"
            SELECT * FROM risk_reduction
            WHERE EXISTS(
                SELECT *
                FROM risk_treatment
                WHERE rap_code = $1 AND tour_code = $2 AND threat_code = $3 AND risk_treatment.treatment_code = risk_reduction.code
            )"#, rap_code, tour_code, threat_code)
                .fetch_all(db)
                .await?
        )
    }

    pub async fn list_risk_transfer(db: &Pool<Postgres>) -> ApiResult<Vec<RiskTransferModel>> {
        Ok(sqlx::query_as!(RiskTransferModel, r#" SELECT * FROM risk_transfer"#).fetch_all(db).await?)
    }

    pub async fn risk_treatment(
        db: &Pool<Postgres>,
        rap_code: String,
        tour_code: String,
        threat_code: String,
    ) -> ApiResult<Option<RiskTreatmentModel>> {
        Ok(
        sqlx::query_as!(RiskTreatmentModel,
            r#"
            SELECT treatment_type, treatment_code
            FROM risk_treatment
            WHERE rap_code = $1 AND tour_code = $2 AND threat_code = $3
            "#,
            rap_code,
            tour_code,
            threat_code,
        )
            .fetch_optional(db)
            .await?
        )
    }
    async fn clear_risk_treatment(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
    ) -> ApiResult<()> {
        sqlx::query(
            r#"DELETE FROM risk_treatment WHERE rap_code = $1 AND tour_code = $2 AND threat_code = $3"#,
        )
            .bind(rap_code)
            .bind(tour_code)
            .bind(threat_code)
            .execute(&mut *tx)
            .await?;

        Ok(())
    }
    async fn add_risk_treatment(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        treatment_type: RiskTreatmentType,
        treatment_code: String,
    ) -> ApiResult<()> {
        sqlx::query(r#"INSERT INTO risk_treatment VALUES ($1,$2,$3,$4,$5)"#)
            .bind(rap_code)
            .bind(tour_code)
            .bind(threat_code)
            .bind(treatment_type)
            .bind(treatment_code)
            .execute(&mut *tx)
            .await?;

        Ok(())
    }
    pub async fn risk_accept(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        acp_code: String,
    ) -> ApiResult<()> {
        Self::clear_risk_treatment(&mut *tx, rap_code.clone(), tour_code.clone(), threat_code.clone()).await?;
        Self::add_risk_treatment(&mut *tx, rap_code, tour_code, threat_code, RiskTreatmentType::Acceptance, acp_code).await?;

        Ok(())
    }

    pub async fn risk_accept_with_create(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        create_model: RiskAcceptanceCreateModel,
    ) -> ApiResult<String> {
        let acp_code = Self::create_risk_acceptance(&mut *tx, create_model).await?;
        Self::risk_accept(&mut *tx, rap_code, tour_code, threat_code, acp_code.clone()).await?;

        Ok(acp_code)
    }
    const RISK_TABLE_NAME: &'static str = "risk_treatment_code";
    const RISK_CODE_DIGITS: usize = 10;
    const RISK_ACCEPTANCE_TABLE_PREFIX: &'static str = "ACP";
    const RISK_AVOIDANCE_TABLE_PREFIX: &'static str = "AVD";

    async fn create_risk_classification_code(
        tx: &mut PgConnection,
        table_prefix: &str
    ) -> ApiResult<String> {
        let code = next_code_like(Self::RISK_TABLE_NAME, table_prefix, Self::RISK_CODE_DIGITS, &mut *tx).await?;

        sqlx::query(r#"INSERT INTO risk_treatment_code VALUES ($1)"#)
            .bind(code.clone())
            .execute(&mut *tx)
            .await?;

        Ok(code)
    }
    pub async fn create_risk_acceptance(
        tx: &mut PgConnection,
        create_model: RiskAcceptanceCreateModel,
    ) -> ApiResult<String> {
        let code = Self::create_risk_classification_code(&mut *tx, Self::RISK_ACCEPTANCE_TABLE_PREFIX).await?;
        sqlx::query(r#"INSERT INTO risk_acceptance VALUES ($1,$2,$3)"#)
            .bind(code.clone())
            .bind(create_model.name)
            .bind(create_model.explanation)
            .execute(&mut *tx)
            .await?;

        Ok(code)
    }

    pub async fn risk_avoid(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        avd_code: String,
    ) -> ApiResult<()> {
        Self::clear_risk_treatment(&mut *tx, rap_code.clone(), tour_code.clone(), threat_code.clone()).await?;
        Self::add_risk_treatment(&mut *tx, rap_code, tour_code, threat_code, RiskTreatmentType::Avoidance, avd_code).await?;

        Ok(())
    }
    pub async fn risk_avoid_with_create(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        create_model: RiskAvoidanceCreateModel,
    ) -> ApiResult<String> {
        let acp_code = Self::create_risk_avoidance(&mut *tx, create_model).await?;
        Self::risk_avoid(&mut *tx, rap_code, tour_code, threat_code, acp_code.clone()).await?;

        Ok(acp_code)
    }

    pub async fn create_risk_avoidance(
        tx: &mut PgConnection,
        create_model: RiskAvoidanceCreateModel,
    ) -> ApiResult<String> {
        let code = Self::create_risk_classification_code(&mut *tx, Self::RISK_AVOIDANCE_TABLE_PREFIX).await?;
        sqlx::query(r#"INSERT INTO risk_avoidance VALUES ($1,$2,$3)"#)
            .bind(code.clone())
            .bind(create_model.name)
            .bind(create_model.explanation)
            .execute(&mut *tx)
            .await?;

        Ok(code)
    }

    pub async fn create_risk_transfer(
        tx: &mut PgConnection,
        create_model: RiskTransferCreateModel,
    ) -> ApiResult<String> {
        let code = Self::create_risk_classification_code(&mut *tx, Self::RISK_ACCEPTANCE_TABLE_PREFIX).await?;
        sqlx::query(r#"INSERT INTO risk_transfer VALUES ($1,$2,$3,$4,$5)"#)
            .bind(code.clone())
            .bind(create_model.name)
            .bind(create_model.risk_transfer_type)
            .bind(create_model.checklist)
            .bind(create_model.explanation)
            .execute(&mut *tx)
            .await?;

        Ok(code)
    }

    pub async fn risk_transfer(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        tsf_code: String,
    ) -> ApiResult<()> {
        Self::clear_risk_treatment(&mut *tx, rap_code.clone(), tour_code.clone(), threat_code.clone()).await?;
        Self::add_risk_treatment(&mut *tx, rap_code, tour_code, threat_code, RiskTreatmentType::Transfer, tsf_code).await?;

        Ok(())
    }
    pub async fn risk_transfer_with_create(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        create_model: RiskTransferCreateModel,
    ) -> ApiResult<String> {
        let trs_code = Self::create_risk_transfer(&mut *tx, create_model).await?;
        Self::risk_transfer(&mut *tx, rap_code, tour_code, threat_code, trs_code.clone()).await?;

        Ok(trs_code)
    }


    pub async fn create_risk_reduction(
        tx: &mut PgConnection,
        create_model: RiskReductionCreateModel,
    ) -> ApiResult<String> {
        let code = Self::create_risk_classification_code(&mut *tx, Self::RISK_ACCEPTANCE_TABLE_PREFIX).await?;
        sqlx::query(r#"INSERT INTO risk_treatment VALUES ($1,$2,$3,$4,$5,$6)"#)
            .bind(code.clone())
            .bind(create_model.name)
            .bind(create_model.confidentiality_protected)
            .bind(create_model.integrity_protected)
            .bind(create_model.availability_protected)
            .bind(create_model.explanation)
            .execute(&mut *tx)
            .await?;

        Ok(code)
    }

    pub async fn risk_reduce(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        red_code_list: Vec<String>,
    ) -> ApiResult<()> {

        Self::clear_risk_treatment(&mut *tx, rap_code.clone(), tour_code.clone(), threat_code.clone()).await?;
        for red_code in red_code_list {
            Self::add_risk_treatment(&mut *tx, rap_code.clone(), tour_code.clone(), threat_code.clone(), RiskTreatmentType::Reduction, red_code).await?;
        }

        Ok(())
    }
    pub async fn risk_reduce_with_create(
        db: &Pool<Postgres>,
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        create_model: RiskReductionCreateModel,
    ) -> ApiResult<String> {
        let risk_reduction_list:Vec<RiskReductionModel> = Self::get_risk_reduction(db, rap_code.clone(), tour_code.clone(), threat_code.clone()).await?;
        let mut red_code_list: Vec<String> = risk_reduction_list.into_iter().map(|red| red.code).collect();

        let red_code = Self::create_risk_reduction(&mut *tx, create_model).await?;
        red_code_list.push(red_code.clone());

        Self::risk_reduce(&mut *tx, rap_code, tour_code, threat_code, red_code_list).await?;

        Ok(red_code)
    }
}
