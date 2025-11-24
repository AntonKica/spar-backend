use crate::model::step_2_threat_identification_models::{TourThreatModel, TourThreatReviewModel, TourThreatSummaryModel};
use crate::service::ApiResult;
use sqlx::{PgConnection, Pool, Postgres};
use crate::enums::step_2_threat_identification_enums::ThreatRelevance;
use crate::enums::step_3_risk_classification_enums::{ThreatImpact, ThreatProbability};
use crate::model::step_3_risk_classification_models::{TourRiskClassificationClassifyModel, TourRiskClassificationModel};

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
}
