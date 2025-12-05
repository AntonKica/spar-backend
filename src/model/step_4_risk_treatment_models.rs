use serde::{Deserialize, Serialize};
use crate::enums::step_4_risk_treatment_enums::RiskTransferType;
use crate::model::asset_model::AssetModel;
use crate::model::risk_analysis_process_models::CodeNameModel;
use crate::model::threat_models::ThreatModel;

#[derive(Serialize)]
pub struct TourRiskClassificationCalculatedModel {
    pub threat_code: String,
    pub threat_name: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
    pub treatment_type: Option<i32>,
    pub risk: i32,
}

#[derive(Deserialize)]
pub struct RiskAcceptanceCreateModel {
    pub name: String,
    pub explanation: String,
}

#[derive(Serialize)]
pub struct RiskAcceptanceModel {
    pub code: String,
    pub name: String,
    pub explanation: String,
}

#[derive(Deserialize)]
pub struct RiskAvoidanceCreateModel {
    pub name: String,
    pub explanation: String,
}

#[derive(Serialize)]
pub struct RiskAvoidanceModel {
    pub code: String,
    pub name: String,
    pub explanation: String,
}

// todo  merge with TourRiskClassificationCalculatedModel?
#[derive(Serialize)]
pub struct RiskTreatmentModel {
    pub treatment_type: i32,
    pub treatment_code: String,
}


#[derive(Deserialize)]
pub struct RiskTransferCreateModel {
    pub name: String,
    pub risk_transfer_type: RiskTransferType,
    pub checklist: Vec<String>,
    pub explanation: String,
}

#[derive(Serialize)]
pub struct RiskTransferModel {
    pub code: String,
    pub name: String,
    pub risk_transfer_type: i32,
    pub checklist: Vec<String>,
    pub explanation: String,
}

#[derive(Deserialize)]
pub struct RiskReductionCreateModel {
    pub name: String,
    pub confidentiality_protected: bool,
    pub integrity_protected: bool,
    pub availability_protected: bool,
    pub explanation: String,
}
#[derive(Serialize)]
pub struct RiskReductionModel {
    pub code: String,
    pub name: String,
    pub confidentiality_protected: bool,
    pub integrity_protected: bool,
    pub availability_protected: bool,
    pub explanation: String,
}

#[derive(Serialize)]
pub struct RiskTreatmentSummary {
    pub threat_summary_list: Vec<CodeNameModel>,
    pub asset_summary_list: Vec<CodeNameModel>,

    pub risk_treatment_matrix: Vec<Vec<Vec<CodeNameModel>>>,
}
