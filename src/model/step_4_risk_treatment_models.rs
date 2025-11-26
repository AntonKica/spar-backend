use serde::{Deserialize, Serialize};
use crate::enums::step_3_risk_classification_enums::{ThreatRisk};

#[derive(Serialize)]
pub struct TourRiskClassificationCalculatedModel {
    pub threat_code: String,
    pub threat_name: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
    pub treatment_type: Option<i32>,
    pub risk: ThreatRisk,
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

// todo  merge with TourRiskClassificationCalculatedModel?
#[derive(Serialize)]
pub struct RiskTreatmentModel {
    pub treatment_type: i32,
    pub treatment_code: String,
}
