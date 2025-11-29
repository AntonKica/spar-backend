use serde::{Deserialize, Serialize};
use crate::enums::step_4_risk_treatment_enums::RiskTransferType;

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