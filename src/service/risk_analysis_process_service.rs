use crate::model::{next_code_for, RiskAnalysisProcessCreateModel, TargetObjectUnderReviewCreateModel};
use crate::response::RiskAnalysisProcessResponse;
use crate::service::{ApiError, ApiResult, GeneralService};
use crate::workflow::create_risk_analysis_process_workflow;
use crate::workflow_model::{create_workflow_model, WorkflowModel};
use chrono::{NaiveDate, Utc};
use serde_json::{json, Value};
use sqlx::{PgConnection, Pool, Postgres};

pub struct RiskAnalysisProcessService;

#[derive(Clone)]
struct RiskAnalysisProcessRow {
    pub code: String,
    pub created_on: NaiveDate,
    pub workflow: Value,
}

pub struct RiskAnalysisProcessModel {
    pub code: String,
    pub created_on: NaiveDate,
    pub workflow: WorkflowModel,
    pub target_objects_under_review: Vec<String>
}

impl RiskAnalysisProcessModel {
    fn from_row(row: RiskAnalysisProcessRow, target_objects_under_review: Vec<TargetObjectUnderReviewCreateModel>) -> RiskAnalysisProcessModel {
        RiskAnalysisProcessModel {
            code: row.code.clone(),
            created_on: row.created_on.clone(),
            // TODO please dont fail
            workflow: serde_json::from_value(row.workflow.clone()).unwrap(),
            target_objects_under_review: target_objects_under_review.into_iter().map(|t| t.asset_code).collect(),
        }
    }
}

impl GeneralService<RiskAnalysisProcessResponse> for RiskAnalysisProcessService {
    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<RiskAnalysisProcessResponse>> {
        let rows = sqlx::query_as!(RiskAnalysisProcessRow, r#"SELECT * FROM risk_analysis_process"#)
            .fetch_all(db)
            .await?;

        let mut list: Vec<RiskAnalysisProcessResponse> = vec![];
        for row in rows {
            let target_objects_under_review = sqlx::query_as!(TargetObjectUnderReviewCreateModel, r#"SELECT * FROM target_object_under_review WHERE risk_analysis_process_code = $1"#, row.code)
                .fetch_all(db)
                .await?;

            list.push(RiskAnalysisProcessResponse::from(RiskAnalysisProcessModel::from_row(row.clone(), target_objects_under_review)));
        };

        Ok(list)
    }

    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<RiskAnalysisProcessResponse> {
        let row = sqlx::query_as!(RiskAnalysisProcessRow, r#"SELECT * FROM risk_analysis_process WHERE code = $1"#, code)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("RiskAnalysisProcess with code {} not found", code)))?;
        let target_objects_under_review = sqlx::query_as!(TargetObjectUnderReviewCreateModel, r#"SELECT * FROM target_object_under_review WHERE risk_analysis_process_code = $1"#, row.code)
            .fetch_all(db)
            .await?;
        Ok(RiskAnalysisProcessResponse::from(RiskAnalysisProcessModel::from_row(row, target_objects_under_review)))
    }
}

impl RiskAnalysisProcessService {
    const TABLE_NAME: &'static str = "risk_analysis_process";
    const CODE_PREFIX: &'static str = "RAP";
    const CODE_DIGITS: usize = 8;
    pub async fn create(
        tx: &mut PgConnection,
        create_model: RiskAnalysisProcessCreateModel
    ) -> ApiResult<String> {
        let code = match next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await {
            Ok(val) => val,
            Err(_) => return Err(ApiError::Internal)
        };
        
        let created_on = Utc::now().date_naive();
        let workflow = json!(create_workflow_model(&create_risk_analysis_process_workflow()));

        sqlx::query!(
        r#"INSERT INTO risk_analysis_process(code, created_on, workflow) VALUES ($1,$2,$3)"#,
            code,
            created_on,
            workflow
        )
            .execute(&mut *tx)
            .await?;


        let query = r"
            INSERT INTO target_object_under_review (
                risk_analysis_process_code,
                asset_code
            ) SELECT * FROM UNNEST(
                $1::CHAR(8)[],
                $2::VARCHAR(20)[]
            ) ON CONFLICT DO NOTHING";

        sqlx::query(query)
            .bind(vec![code.to_owned(); create_model.target_objects_under_review.len()])
            .bind(&create_model.target_objects_under_review)
            .fetch_all(&mut *tx)
            .await?;

        Ok(code)
    }
}