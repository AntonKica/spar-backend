use chrono::Utc;
use sqlx::{PgConnection, Pool, Postgres};
use crate::enums::risk_analysis_process_enums::ProcessStatus;
use crate::model::risk_analysis_process_models::RiskAnalysisProcessModel;
use crate::model::RiskAnalysisProcessCreateModel;
use crate::service::{next_code_for, ApiError, ApiResult};

pub struct RiskAnalysisProcessService;

impl RiskAnalysisProcessService {
    const TABLE_NAME: &'static str = "risk_analysis_process";
    const CODE_PREFIX: &'static str = "RAP";
    const CODE_DIGITS: usize = 10;
    pub async fn create(
        tx: &mut PgConnection,
    ) -> ApiResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;
        let created_on = Utc::now().date_naive();

        sqlx::query!(
        r#"INSERT INTO risk_analysis_process VALUES ($1,$2,$3,$4,$5,$6,$7,$8)"#,
            code,
            created_on,
            ProcessStatus::InProgress as i32,
            ProcessStatus::InProgress as i32,
            ProcessStatus::Waiting as i32,
            ProcessStatus::Waiting as i32,
            ProcessStatus::Waiting as i32,
            ProcessStatus::Waiting as i32,
        )
            .execute(&mut *tx)
            .await?;
        Ok(code)
    }

    pub async fn get_by_code(
        db: &Pool<Postgres>,
        rap_code: String) -> ApiResult<RiskAnalysisProcessModel> {
        let row = sqlx::query!(r#"SELECT * FROM risk_analysis_process WHERE code = $1"#, rap_code.clone())
            .fetch_optional(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("Risk analysis process {rap_code} not found")))?;

        let tour_list = sqlx::query!(r#"SELECT asset_code FROM risk_analysis_process_tour_list WHERE risk_analysis_process_code = $1"#, rap_code)
            .fetch_all(db)
            .await?
            .into_iter()
            .map(|tl| tl.asset_code)
            .collect();


        Ok(RiskAnalysisProcessModel {
            code: row.code,
            created_on: row.created_on,
            process_status: row.process_status,
            step_1_select_tour_process_status: row.step_1_select_tour_process_status,
            step_2_threat_identification_process_status: row.step_2_threat_identification_process_status,
            step_3_risk_analysis_process_status: row.step_3_risk_analysis_process_status,
            step_4_risk_treatment_process_status: row.step_4_risk_treatment_process_status,
            step_5_risk_treatment_check_process_status: row.step_5_risk_treatment_check_process_status,
            tour_list: tour_list,
        })
    }

    pub async fn set_tour(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code_list: Vec<String>
    ) -> ApiResult<()> {
        sqlx::query!(
            r#"DELETE FROM risk_analysis_process_tour_list WHERE risk_analysis_process_code = $1"#,
            rap_code.clone()
        )
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            r#"
            INSERT INTO risk_analysis_process_tour_list
            SELECT * FROM UNNEST(
                $1::CHAR(10)[],
                $2::CHAR(10)[]
            );"#
        )
            .bind(vec![rap_code.clone(); tour_code_list.len()])
            .bind(tour_code_list)
            .fetch_all(&mut *tx)
            .await?;

        Ok(())
    }
}
