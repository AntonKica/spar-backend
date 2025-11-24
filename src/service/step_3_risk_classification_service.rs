use crate::model::step_2_threat_identification_models::{TourThreatIdentificationModel, TourThreatModel, TourThreatReviewModel, TourThreatSummaryModel};
use crate::service::ApiResult;
use sqlx::{PgConnection, Pool, Postgres};
use sqlx::postgres::PgRow;
use crate::enums::risk_analysis_process_enums::ProcessStep::Step2RelevantThreatIdentification;
use crate::enums::step_2_threat_identification_enums::ThreatRelevance;
use crate::enums::step_3_risk_classification_enums::{ThreatImpact, ThreatProbability, ThreatRisk};
use crate::model::step_3_risk_classification_models::{RiskClassificationSummaryModel, RiskSummaryModel, TourRiskClassificationClassifyModel, TourRiskClassificationModel, TourRiskClassificationSummaryModel};
use crate::model::threat_models::ThreatModel;
use crate::service::step_2_threat_idenfication_service::Step2ThreatIdentificationService;

pub struct Step3RiskClassificationService;

impl Step3RiskClassificationService {
    pub async fn initialize_step(
        tx: &mut PgConnection,
        rap_code: String,
    ) -> ApiResult<()> {
        sqlx::query(r#"
INSERT INTO risk_classification
SELECT rap_code, tour_code, threat_code, $2, $3, ''
FROM tour_threat_list
WHERE rap_code = $1
"#
        )
            .bind(rap_code)
            .bind(ThreatProbability::Rarely)
            .bind(ThreatImpact::Negligible)
            .execute(&mut *tx)
            .await?;
        Ok(())
    }

    pub async fn tour_risk_classification_list(
        db: &Pool<Postgres>,
        rap_code: String,
        tour_code: String,
    ) -> ApiResult<Vec<TourRiskClassificationModel>> {
        Ok(sqlx::query_as!(TourRiskClassificationModel, r#"
            SELECT
                threat.code as threat_code,
                threat.name as threat_name,
                confidentiality_impaired,
                availability_impaired,
                integrity_impaired,
                probability,
                impact,
                evaluation
            FROM risk_classification
            INNER JOIN threat ON threat.code = risk_classification.threat_code
            WHERE rap_code = $1 AND tour_code = $2
            "#, rap_code.clone(), tour_code.clone())
            .fetch_all(db)
            .await?
        )
    }
    
    pub async fn threat_classify(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        classify: TourRiskClassificationClassifyModel,
    ) -> ApiResult<()> {
        sqlx::query(r#"
            UPDATE risk_classification
            SET
                probability = $4,
                impact = $5,
                evaluation = $6
            WHERE rap_code = $1 AND tour_code = $2 AND threat_code = $3
            "#,
        )
            .bind(rap_code)
            .bind(tour_code)
            .bind(threat_code)
            .bind(classify.probability)
            .bind(classify.impact)
            .bind(classify.evaluation)
            .execute(&mut *tx)
            .await?;

        Ok(())
    }


    fn risk_matrix(probability_val: i32, impact_val: i32) -> ThreatRisk {
        let probability: ThreatProbability = ThreatProbability::from(probability_val);
        let impact = ThreatImpact::from(impact_val);

        match probability {
            ThreatProbability::Rarely => {
                match impact {
                    ThreatImpact::Negligible => ThreatRisk::Low,
                    ThreatImpact::Limited => ThreatRisk::Low,
                    ThreatImpact::Significant => ThreatRisk::Medium,
                    ThreatImpact::LifeThreatening => ThreatRisk::Medium
                }
            }
            ThreatProbability::Medium => {
                match impact {
                    ThreatImpact::Negligible => ThreatRisk::Low,
                    ThreatImpact::Limited => ThreatRisk::Low,
                    ThreatImpact::Significant => ThreatRisk::Medium,
                    ThreatImpact::LifeThreatening => ThreatRisk::High
                }
            }
            ThreatProbability::Often => {
                match impact {
                    ThreatImpact::Negligible => ThreatRisk::Low,
                    ThreatImpact::Limited => ThreatRisk::Medium,
                    ThreatImpact::Significant => ThreatRisk::High,
                    ThreatImpact::LifeThreatening => ThreatRisk::VeryHigh
                }
            }
            ThreatProbability::VeryOften => {
                match impact {
                    ThreatImpact::Negligible => ThreatRisk::Low,
                    ThreatImpact::Limited => ThreatRisk::High,
                    ThreatImpact::Significant => ThreatRisk::VeryHigh,
                    ThreatImpact::LifeThreatening => ThreatRisk::VeryHigh
                }
            }
        }
    }

    pub async fn risk_summary_list(
        db: &Pool<Postgres>,
        rap_code: String,
        tour_code: String,
    ) -> ApiResult<Vec<RiskSummaryModel>> {
        let risk_list: Vec<(String, String, i32, i32)> = sqlx::query_as(r#"
            SELECT
                threat.code as threat_code,
                threat.name as threat_name,
                probability,
                impact
            FROM risk_classification
            INNER JOIN threat ON threat.code = risk_classification.threat_code
            WHERE rap_code = $1 AND tour_code = $2
            "#)
                .bind(rap_code.clone())
                .bind(tour_code.clone())
                .fetch_all(db)
                .await?;
        
        Ok(
            risk_list.into_iter().map(|(threat_code, threat_name, probability, impact)| {
                RiskSummaryModel {
                    threat_code: threat_code,
                    threat_name: threat_name,
                    risk: Self::risk_matrix(probability, impact)
                }
            }).collect()
        )
    }
    pub async fn risk_classification_summary(
        db: &Pool<Postgres>,
        rap_code: String,
    ) -> ApiResult<RiskClassificationSummaryModel> {
        let mut tour_risk_classification_list: Vec<TourRiskClassificationSummaryModel> = Vec::new();
        for tour in Step2ThreatIdentificationService::tour_list(&db, rap_code.clone()).await? {
            tour_risk_classification_list.push(TourRiskClassificationSummaryModel {
                tour_code: tour.code.clone(),
                tour_name: tour.name.clone(),
                threat_risk_list: Step3RiskClassificationService::risk_summary_list(&db, rap_code.clone(), tour.code.clone()).await?,
            })
        }

        Ok(RiskClassificationSummaryModel{
            tour_risk_classification_list,
            threat_list: Step2ThreatIdentificationService::threat_list(&db, rap_code).await?,
        })
    }
}
