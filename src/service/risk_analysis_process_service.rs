use crate::enums::risk_analysis_process_enums::ProcessStep;
use chrono::Utc;
use sqlx::{PgConnection, Pool, Postgres};
use sqlx::Error::Protocol;
use crate::enums::risk_analysis_process_enums::ProcessStatus;
use crate::model::risk_analysis_process_models::{RiskAnalysisProcessDetailModel, RiskAnalysisProcessModel};
use crate::model::RiskAnalysisProcessCreateModel;
use crate::service::{next_code_for, ApiError, ApiResult};
use crate::service::asset_service::AssetService;

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

        sqlx::query(r#"INSERT INTO risk_analysis_process VALUES ($1,$2,$3,$4)"#)
            .bind(code.clone())
            .bind(created_on)
            .bind(ProcessStatus::InProgress)
            .bind(ProcessStep::Step1SelectTour as i32)
            .execute(&mut *tx)
            .await?;
        Ok(code)
    }

    pub async fn detail(
        db: &Pool<Postgres>,
        rap_code: String) -> ApiResult<RiskAnalysisProcessDetailModel> {
        let rap = sqlx::query_as!(RiskAnalysisProcessModel, r#"SELECT * FROM risk_analysis_process WHERE code = $1"#, rap_code.clone())
            .fetch_optional(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("Risk analysis process {rap_code} not found")))?;

        let tour_list = AssetService::list_for_risk_analysis_process(&db, rap_code.clone()).await?;


        Ok(RiskAnalysisProcessDetailModel {
            code: rap.code,
            created_on: rap.created_on,
            process_status: rap.process_status,
            process_step: rap.process_step,
            tour_list,
        })
    }

    pub async fn list(
        db: &Pool<Postgres>,
    ) -> ApiResult<Vec<RiskAnalysisProcessModel>> {
        Ok(sqlx::query_as!(RiskAnalysisProcessModel, r#"SELECT * FROM risk_analysis_process"#).fetch_all(db) .await?)
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

    pub async fn step_complete(
        tx: &mut PgConnection,
        rap_code: String,
        process_step: ProcessStep,
    ) -> ApiResult<()> {
        let next_step = match process_step {
            ProcessStep::Step1SelectTour => ProcessStep::Step2RelevantThreatIdentification,
            ProcessStep::Step2RelevantThreatIdentification => ProcessStep::Step3RiskClassification,
            ProcessStep::Step3RiskClassification => ProcessStep::Step4RiskTreatment,
            ProcessStep::Step4RiskTreatment => ProcessStep::Step5RiskTreatmentCheck,
            ProcessStep::Step5RiskTreatmentCheck => ProcessStep::Step6Finished,
            ProcessStep::Step6Finished => ProcessStep::Step6Finished
        };

        let next_status = match next_step {
            ProcessStep::Step6Finished => ProcessStatus::Finished,
            _ => ProcessStatus::InProgress
        };

        sqlx::query(
            r#"UPDATE risk_analysis_process SET process_status = $2, process_step = $3  WHERE code = $1"#
        )
            .bind(rap_code.clone())
            .bind(next_status)
            .bind(next_step)
            .execute(&mut *tx)
            .await?;


        Ok(())
    }
}
