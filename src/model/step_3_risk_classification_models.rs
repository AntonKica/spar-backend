use serde::{Deserialize, Serialize};
use crate::enums::step_3_risk_classification_enums::{ThreatImpact, ThreatProbability};

#[derive(Serialize)]
pub struct TourRiskClassificationModel {
    pub threat_code: String,
    pub threat_name: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,

    pub probability: i32,
    pub impact: i32,
    pub evaluation: String,
}

#[derive(Deserialize)]
pub struct TourRiskClassificationClassifyModel {
    pub probability: ThreatProbability,
    pub impact: ThreatImpact,
    pub evaluation: String,
}