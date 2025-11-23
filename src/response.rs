use chrono::NaiveDate;
use serde::Serialize;
use crate::workflow_model::WorkflowModel;


#[derive(Serialize)]
pub struct RiskAnalysisProcessResponse {
    pub code: String,
    pub created_on: NaiveDate,
    pub workflow: WorkflowModel,
}