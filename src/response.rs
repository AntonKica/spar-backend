use chrono::NaiveDate;
use serde::Serialize;
use crate::model::{ApplicationModel, BusinessProcessModel, ITSystemModel, RoleModel};
use crate::service::risk_analysis_process_service::RiskAnalysisProcessModel;
use crate::workflow_model::WorkflowModel;

#[derive(Serialize)]
pub struct EnumResponse {
    pub code: i32,
    pub name: String,
}

#[derive(Serialize)]
pub struct BusinessProcessResponse {
    code: String,
    name: String,
    description: String,
    process_type: EnumResponse,
    responsible: Option<String>
}

impl From<BusinessProcessModel> for BusinessProcessResponse {
    fn from(model: BusinessProcessModel) -> Self {
        Self {
            code: model.code,
            name: model.name,
            description: model.description,
            process_type: EnumResponse::from(model.process_type),
            responsible: model.responsible,
        }
    }
}

#[derive(Serialize)]
pub struct RoleResponse {
    code: String,
    name: String,
    description: String,
}

impl From<RoleModel> for RoleResponse {
    fn from(model: RoleModel) -> Self {
        Self {
            code: model.code,
            name: model.name,
            description: model.description,
        }
    }
}

#[derive(Serialize)]
pub struct ApplicationResponse {
    code: String,
    name: String,
    description: String,
    module_type: EnumResponse,
    application_user: String,
    responsible: String,
}

impl From<ApplicationModel> for ApplicationResponse {
    fn from(model: ApplicationModel) -> Self {
        Self {
            code: model.code,
            name: model.name,
            description: model.description,
            module_type: EnumResponse::from(model.module_type),
            application_user: model.application_user,
            responsible: model.responsible,
        }
    }
}
#[derive(Serialize)]
pub struct ITSystemResponse {
    code: String,
    name: String,
    description: String,
    count: i32,
    module_type: EnumResponse,
    application_user: String,
    responsible: String,
}

impl From<ITSystemModel> for ITSystemResponse {
    fn from(model: ITSystemModel) -> Self {
        Self {
            code: model.code,
            name: model.name,
            description: model.description,
            count: model.count,
            module_type: EnumResponse::from(model.module_type),
            application_user: model.application_user,
            responsible: model.responsible,
        }
    }
}

#[derive(Serialize)]
pub struct RiskAnalysisProcessResponse {
    pub code: String,
    pub created_on: NaiveDate,
    pub workflow: WorkflowModel,
}

impl From<RiskAnalysisProcessModel> for RiskAnalysisProcessResponse {
    fn from(model: RiskAnalysisProcessModel) -> Self {
        Self {
            code: model.code,
            created_on: model.created_on,
            workflow: model.workflow,
        }
    }
}
