use serde::Serialize;
use sqlx::{PgConnection, Pool, Postgres, Row};
use crate::enums::risk_classification_enums::{FrequencyOfOccurrence, PotentialDamage, PotentialRisk};
use crate::enums::risk_treatment_enums::RiskTreatment;
use crate::model::risk_classsification_model::TOURSpecificThreatRiskClassificationUpdateModel;
use crate::model::risk_treatment_model::{TOURElementaryThreatRiskTreatmentModel, TOURSpecificThreatRiskTreatmentModel, TOURThreatRiskTreatmentUpdateModel};
use crate::service::ApiResult;

pub struct RiskTreatmentService{}

#[derive(Serialize)]
pub struct TOURElementaryThreatRiskTreatmentResponse {
    tour_elementary_threat_code: String,
    potential_risk: i32,
    remaining_risk: i32,
    risk_treatment: i32,
    description: String,
}

impl From<TOURElementaryThreatRiskTreatmentModel> for TOURElementaryThreatRiskTreatmentResponse {
    fn from(model: TOURElementaryThreatRiskTreatmentModel) -> Self {
        Self {
            tour_elementary_threat_code: model.tour_elementary_threat_code.to_owned(),
            potential_risk: model.potential_risk,
            remaining_risk: model.remaining_risk,
            risk_treatment: model.risk_treatment,
            description: model.description.to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct TOURSpecificThreatRiskTreatmentResponse {
    tour_specific_threat_code: String,
    potential_risk: i32,
    remaining_risk: i32,
    risk_treatment: i32,
    description: String,
}

impl From<TOURSpecificThreatRiskTreatmentModel> for TOURSpecificThreatRiskTreatmentResponse {
    fn from(model: TOURSpecificThreatRiskTreatmentModel) -> Self {
        Self {
            tour_specific_threat_code: model.tour_specific_threat_code.to_owned(),
            potential_risk: model.potential_risk,
            remaining_risk: model.remaining_risk,
            risk_treatment: model.risk_treatment,
            description: model.description.to_owned(),
        }
    }
}
#[derive(Serialize)]
pub struct TOURRiskTreatmentListResponse {
    tour_elementary_threat_risk_treatment_list: Vec<TOURElementaryThreatRiskTreatmentResponse>,
    tour_specific_threat_risk_treatment_list: Vec<TOURSpecificThreatRiskTreatmentResponse>,
}
struct RiskClassificationRow {
    pub asset_code: String,
    pub threat_code: String,
    pub frequency_of_occurrence: i32,
    pub potential_damage: i32,
}
fn risk_matrix(row: &RiskClassificationRow) -> i32 {
    let frequency = FrequencyOfOccurrence::from(row.frequency_of_occurrence);
    let damage = PotentialDamage::from(row.potential_damage);

    let res = match frequency {
        FrequencyOfOccurrence::Rarely => {
            match damage {
                PotentialDamage::Negligible => PotentialRisk::Low,
                PotentialDamage::Limited => PotentialRisk::Low,
                PotentialDamage::Significant => PotentialRisk::Medium,
                PotentialDamage::LifeThreatening => PotentialRisk::Medium
            }
        }
        FrequencyOfOccurrence::Medium => {
            match damage {
                PotentialDamage::Negligible => PotentialRisk::Low,
                PotentialDamage::Limited => PotentialRisk::Low,
                PotentialDamage::Significant => PotentialRisk::Medium,
                PotentialDamage::LifeThreatening => PotentialRisk::High
            }
        }
        FrequencyOfOccurrence::Often => {
            match damage {
                PotentialDamage::Negligible => PotentialRisk::Low,
                PotentialDamage::Limited => PotentialRisk::Medium,
                PotentialDamage::Significant => PotentialRisk::High,
                PotentialDamage::LifeThreatening => PotentialRisk::VeryHigh
            }
        }
        FrequencyOfOccurrence::VeryOften => {
            match damage {
                PotentialDamage::Negligible => PotentialRisk::Low,
                PotentialDamage::Limited => PotentialRisk::High,
                PotentialDamage::Significant => PotentialRisk::VeryHigh,
                PotentialDamage::LifeThreatening => PotentialRisk::VeryHigh
            }
        }
    };

    res as i32
}

impl RiskTreatmentService {
    pub async fn create_risk_treatments(tx: &mut PgConnection, rap: String) -> ApiResult<()> {
        let risk_classification_list: Vec<RiskClassificationRow> = sqlx::query_as!(RiskClassificationRow, r#"
SELECT
    asset_code,
    tour_elementary_threat_code as threat_code,
    frequency_of_occurrence,
    potential_damage
FROM tour_elementary_threat_risk_classification
        "#)
            .fetch_all(&mut *tx)
            .await?;

        sqlx::query(r#"
            INSERT INTO tour_elementary_threat_risk_treatment (
                risk_analysis_process_code,
                asset_code,
                tour_elementary_threat_code,
                potential_risk,
                remaining_risk,
                risk_treatment,
                description
            ) SELECT * FROM UNNEST(
                $1::CHAR(8)[],
                $2::VARCHAR(20)[],
                $3::VARCHAR(20)[],
                $4::INTEGER[],
                $4::INTEGER[],
                $5::INTEGER[],
                $6::TEXT[]
            )
            "#
        )
            .bind(vec![rap.to_owned(); risk_classification_list.len()])
            .bind(risk_classification_list.iter().map(|i| i.asset_code.clone()).collect::<Vec<String>>())
            .bind(risk_classification_list.iter().map(|i| i.threat_code.clone()).collect::<Vec<String>>())
            .bind(risk_classification_list.iter().map(risk_matrix).collect::<Vec<i32>>())
            .bind(vec![RiskTreatment::Acceptance as i32; risk_classification_list.len()])
            .bind(vec![""; risk_classification_list.len()])
            .fetch_all(&mut *tx)
            .await?;

        let risk_classification_list: Vec<RiskClassificationRow> = sqlx::query_as!(RiskClassificationRow, r#"
SELECT
    asset_code,
    tour_specific_threat_code as threat_code,
    frequency_of_occurrence,
    potential_damage
FROM tour_specific_threat_risk_classification
        "#)
            .fetch_all(&mut *tx)
            .await?;

        sqlx::query(r#"
            INSERT INTO tour_specific_threat_risk_treatment (
                risk_analysis_process_code,
                asset_code,
                tour_specific_threat_code,
                potential_risk,
                remaining_risk,
                risk_treatment,
                description
            ) SELECT * FROM UNNEST(
                $1::CHAR(8)[],
                $2::VARCHAR(20)[],
                $3::VARCHAR(20)[],
                $4::INTEGER[],
                $4::INTEGER[],
                $5::INTEGER[],
                $6::TEXT[]
            )
            "#
        )
            .bind(vec![rap.to_owned(); risk_classification_list.len()])
            .bind(risk_classification_list.iter().map(|i| i.asset_code.clone()).collect::<Vec<String>>())
            .bind(risk_classification_list.iter().map(|i| i.threat_code.clone()).collect::<Vec<String>>())
            .bind(risk_classification_list.iter().map(risk_matrix).collect::<Vec<i32>>())
            .bind(vec![RiskTreatment::Acceptance as i32; risk_classification_list.len()])
            .bind(vec![""; risk_classification_list.len()])
            .fetch_all(&mut *tx)
            .await?;

        Ok(())
    }

    pub async fn get_risk_treatment_list(db: &Pool<Postgres>, rap: String, asset: String) -> ApiResult<TOURRiskTreatmentListResponse>
    {
        let elementary_threat_risk_treatment_list: Vec<TOURElementaryThreatRiskTreatmentModel> =  sqlx::query_as!(TOURElementaryThreatRiskTreatmentModel,
            r#"
            SELECT * FROM tour_elementary_threat_risk_treatment
            WHERE risk_analysis_process_code = $1 AND asset_code = $2
            ORDER BY tour_elementary_threat_code
            "#,
            rap.clone(),
            asset.clone(),
        ).fetch_all(db).await?;

        let specific_threat_risk_treatment_list: Vec<TOURSpecificThreatRiskTreatmentModel> =  sqlx::query_as!(TOURSpecificThreatRiskTreatmentModel,
            r#"
            SELECT * FROM tour_specific_threat_risk_treatment
            WHERE risk_analysis_process_code = $1 AND asset_code = $2
            ORDER BY tour_specific_threat_code
            "#,
            rap.clone(),
            asset.clone(),
        ).fetch_all(db).await?;

        Ok(TOURRiskTreatmentListResponse{
            tour_elementary_threat_risk_treatment_list: elementary_threat_risk_treatment_list.into_iter().map(TOURElementaryThreatRiskTreatmentResponse::from).collect(),
            tour_specific_threat_risk_treatment_list: specific_threat_risk_treatment_list.into_iter().map(TOURSpecificThreatRiskTreatmentResponse::from).collect(),
        })
    }

    pub async fn update_risk_treatment_specific_threat(db: &Pool<Postgres>, rap: String, asset: String, threat: String, update: TOURThreatRiskTreatmentUpdateModel) -> ApiResult<()>
    {
        sqlx::query!(
        r#"UPDATE tour_specific_threat_risk_treatment
        SET potential_risk = $4, remaining_risk = $5, risk_treatment = $6, description = $7
        WHERE risk_analysis_process_code = $1 AND asset_code = $2 AND tour_specific_threat_code = $3"#,
            rap,
            asset,
            threat,
            update.potential_risk,
            update.remaining_risk,
            update.risk_treatment,
            update.description,
        )
            .execute(db)
            .await?;
        Ok(())
    }
    pub async fn update_risk_treatment_elementary_threat(db: &Pool<Postgres>, rap: String, asset: String, threat: String, update: TOURThreatRiskTreatmentUpdateModel) -> ApiResult<()>
    {
        sqlx::query!(
        r#"UPDATE tour_elementary_threat_risk_treatment
        SET potential_risk = $4, remaining_risk = $5, risk_treatment = $6, description = $7
        WHERE risk_analysis_process_code = $1 AND asset_code = $2 AND tour_elementary_threat_code = $3"#,
            rap,
            asset,
            threat,
            update.potential_risk,
            update.remaining_risk,
            update.risk_treatment,
            update.description,
        )
            .execute(db)
            .await?;
        Ok(())
    }
}