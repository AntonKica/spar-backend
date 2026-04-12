use serde::{Deserialize, Serialize};
use crate::enums::step_2_threat_identification_enums::ThreatRelevance;
use crate::model::risk_analysis_process_models::CodeNameModel;

#[derive(Deserialize)]
pub struct TourThreatReviewModel {
    pub relevance: ThreatRelevance,
    pub explanation: String,
}

#[derive(Serialize)]
pub struct TourModel {
    pub code: String,
    pub name: String,
}
#[derive(Serialize)]
pub struct TourThreatModel {
    pub threat_code: String,
    pub threat_name: String,
    pub relevance: i32,
    pub explanation: String,
}
#[derive(Serialize)]
pub struct TourThreatIdentificationModel {
    pub tour_code: String,
    pub tour_name: String,
    pub threat_list: Vec<TourThreatModel>,
}

#[derive(Serialize)]
pub struct TourThreatSummaryModel {
    pub threat_list: Vec<CodeNameModel>,
    pub tour_threat_list: Vec<TourThreatIdentificationModel>,
}