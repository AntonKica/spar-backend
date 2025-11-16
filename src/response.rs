use chrono::NaiveDate;
use serde::Serialize;
use crate::workflow_model::WorkflowModel;

#[derive(Serialize)]
pub struct EnumResponse {
    pub code: i32,
    pub name: String,
}

#[derive(Serialize)]
pub struct RiskAnalysisProcessResponse {
    pub code: String,
    pub created_on: NaiveDate,
    pub workflow: WorkflowModel,
}

/*
impl From<RiskAnalysisProcessModel> for RiskAnalysisProcessResponse {
    fn from(model: RiskAnalysisProcessModel) -> Self {
        Self {
            code: model.code,
            created_on: model.created_on,
            workflow: model.workflow,
        }
    }
}
 */