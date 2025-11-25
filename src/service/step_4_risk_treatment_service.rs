use crate::service::ApiResult;
use sqlx::{PgConnection, Pool, Postgres};

pub struct Step4RiskTreatmentService;

impl Step4RiskTreatmentService {
    pub async fn initialize_step(
        tx: &mut PgConnection,
        rap_code: String,
    ) -> ApiResult<()> {
        Ok(())
    }

    pub async fn risk_treatment(
        db: &Pool<Postgres>,
        rap_code: String,
        tour_code: String,
        threat_code: String,
    ) -> ApiResult<()> {
        Ok(())
    }


    pub async fn risk_acceptance(
        db: &Pool<Postgres>,
        rap_code: String,
        tour_code: String,
        threat_code: String,
    ) -> ApiResult<()> {
        Ok(())
    }
    pub async fn risk_accept(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        classify: (),
    ) -> ApiResult<()> {
        Ok(())
    }
}
