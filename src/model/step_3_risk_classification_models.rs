use serde::{Deserialize, Serialize};
use crate::enums::step_3_risk_classification_enums::{ThreatImpact, ThreatProbability, ThreatRisk};
use crate::model::threat_models::ThreatModel;

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

#[derive(Serialize)]
pub struct RiskSummaryModel {
    pub threat_code: String,
    pub threat_name: String,
    pub risk: ThreatRisk,
}
#[derive(Serialize)]
pub struct TourRiskClassificationSummaryModel {
    pub tour_code: String,
    pub tour_name: String,
    pub threat_risk_list: Vec<RiskSummaryModel>,
}
#[derive(Serialize)]
pub struct RiskClassificationSummaryModel {
    pub threat_list: Vec<ThreatModel>,
    pub tour_risk_classification_list: Vec<TourRiskClassificationSummaryModel>,
}