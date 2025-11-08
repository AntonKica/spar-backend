use serde::Serialize;
use sqlx::{PgConnection, Pool, Postgres};
use crate::enums::ElementaryThreatRelevance;
use crate::enums::risk_classification_enums::{FrequencyOfOccurrence, PotentialDamage, PotentialRisk};
use crate::model::risk_classsification_model::{TOURElementaryThreatRiskClassificationModel, TOURElementaryThreatRiskClassificationUpdateModel, TOURSpecificThreatRiskClassificationModel, TOURSpecificThreatRiskClassificationUpdateModel};
use crate::model::TOURSpecificThreatModel;
use crate::response::RiskAnalysisProcessResponse;
use crate::service::ApiResult;
use crate::service::risk_analysis_process_service::RiskAnalysisProcessModel;

pub struct RiskClassificationService{}

#[derive(Serialize)]
pub struct TOURElementaryThreatRiskClassificationResponse {
    tour_elementary_threat_code: String,
    frequency_of_occurrence: i32,
    potential_damage: i32,
    description: String,
    evaluation: String,
}

impl From<TOURElementaryThreatRiskClassificationModel> for TOURElementaryThreatRiskClassificationResponse {
    fn from(model: TOURElementaryThreatRiskClassificationModel) -> Self {
        Self {
            tour_elementary_threat_code: model.tour_elementary_threat_code.to_owned(),
            frequency_of_occurrence: model.frequency_of_occurrence,
            potential_damage: model.potential_damage,
            description: model.description.to_owned(),
            evaluation: model.evaluation.to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct TOURSpecificThreatRiskClassificationResponse {
    tour_specific_threat_code: String,
    frequency_of_occurrence: i32,
    potential_damage: i32,
    description: String,
    evaluation: String,
}

impl From<TOURSpecificThreatRiskClassificationModel> for TOURSpecificThreatRiskClassificationResponse {
    fn from(model: TOURSpecificThreatRiskClassificationModel) -> Self {
        Self {
            tour_specific_threat_code: model.tour_specific_threat_code.to_owned(),
            frequency_of_occurrence: model.frequency_of_occurrence,
            potential_damage: model.potential_damage,
            description: model.description.to_owned(),
            evaluation: model.evaluation.to_owned(),
        }
    }
}
#[derive(Serialize)]
pub struct TOURRiskClassificationListResponse {
    tour_elementary_threat_risk_classification_list: Vec<TOURElementaryThreatRiskClassificationResponse>,
    tour_specific_threat_risk_classification_list: Vec<TOURSpecificThreatRiskClassificationResponse>,
}

impl RiskClassificationService {
    pub async fn create_risk_classifications(tx: &mut PgConnection, rap: String) -> ApiResult<()> {
        sqlx::query!(r#"
INSERT INTO tour_elementary_threat_risk_classification
SELECT risk_analysis_process_code,
       asset_code,
       it_grundschutz_elementary_threat_code,
       $3,
       $4,
       '',
       ''
FROM tour_elementary_threat
WHERE risk_analysis_process_code = $1 AND relevance <> $2
        "#,
                    rap,
                    ElementaryThreatRelevance::IRRELEVANT as i32,
                    FrequencyOfOccurrence::Rarely as i32,
                    PotentialDamage::Negligible as i32
        )
            .execute(&mut *tx)
            .await?;

        sqlx::query!(r#"
INSERT INTO tour_specific_threat_risk_classification
SELECT risk_analysis_process_code,
       asset_code,
       code,
       $2,
       $3,
       '',
       ''
FROM tour_specific_threat
WHERE risk_analysis_process_code = $1
        "#,
                    rap,
                    FrequencyOfOccurrence::Rarely as i32,
                    PotentialDamage::Negligible as i32
        )
            .execute(&mut *tx)
            .await?;

        Ok(())
    }

    pub async fn get_risk_classification_list(db: &Pool<Postgres>, rap: String, asset: String) -> ApiResult<TOURRiskClassificationListResponse>
    {
        let elementary_threat_risk_classification_list: Vec<TOURElementaryThreatRiskClassificationModel> =  sqlx::query_as!(TOURElementaryThreatRiskClassificationModel,
            r#"
            SELECT * FROM tour_elementary_threat_risk_classification
            WHERE risk_analysis_process_code = $1 AND asset_code = $2
            ORDER BY tour_elementary_threat_code
            "#,
            rap.clone(),
            asset.clone(),
        ).fetch_all(db).await?;

        let specific_threat_risk_classification_list: Vec<TOURSpecificThreatRiskClassificationModel> =  sqlx::query_as!(TOURSpecificThreatRiskClassificationModel,
            r#"
            SELECT * FROM tour_specific_threat_risk_classification
            WHERE risk_analysis_process_code = $1 AND asset_code = $2
            ORDER BY tour_specific_threat_code
            "#,
            rap.clone(),
            asset.clone(),
        ).fetch_all(db).await?;

        Ok(TOURRiskClassificationListResponse{
            tour_elementary_threat_risk_classification_list: elementary_threat_risk_classification_list.into_iter().map(TOURElementaryThreatRiskClassificationResponse::from).collect(),
            tour_specific_threat_risk_classification_list: specific_threat_risk_classification_list.into_iter().map(TOURSpecificThreatRiskClassificationResponse::from).collect(),
        })
    }
    pub async fn update_risk_classification_elementary_threat(db: &Pool<Postgres>, rap: String, asset: String, threat: String, update: TOURElementaryThreatRiskClassificationUpdateModel) -> ApiResult<()>
    {
        sqlx::query!(
        r#"UPDATE tour_elementary_threat_risk_classification
        SET frequency_of_occurrence = $4, potential_damage = $5, description = $6, evaluation = $7
        WHERE risk_analysis_process_code = $1 AND asset_code = $2 AND tour_elementary_threat_code = $3"#,
            rap,
            asset,
            threat,
            update.frequency_of_occurrence,
            update.potential_damage,
            update.description,
            update.evaluation,
        )
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn update_risk_classification_specific_threat(db: &Pool<Postgres>, rap: String, asset: String, threat: String, update: TOURSpecificThreatRiskClassificationUpdateModel) -> ApiResult<()>
    {
        sqlx::query!(
        r#"UPDATE tour_specific_threat_risk_classification
        SET frequency_of_occurrence = $4, potential_damage = $5, description = $6, evaluation = $7
        WHERE risk_analysis_process_code = $1 AND asset_code = $2 AND tour_specific_threat_code = $3"#,
            rap,
            asset,
            threat,
            update.frequency_of_occurrence,
            update.potential_damage,
            update.description,
            update.evaluation,
        )
            .execute(db)
            .await?;
        Ok(())
    }
}