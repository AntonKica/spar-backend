use serde::{Deserialize, Serialize};
use crate::enums::step_2_threat_identification_enums::ElementaryThreatRelevance;

#[derive(Deserialize)]
pub struct TourEtReviewModel {
    pub relevance: ElementaryThreatRelevance,
    pub explanation: String,
}

#[derive(Deserialize)]
pub struct TourStReviewModel {
    pub relevant: bool,
    pub explanation: String,
}

#[derive(Serialize)]
pub struct TourEtModel {
    pub rap_code: String,
    pub tour_code: String,
    pub et_code: String,
    pub relevance: i32,
    pub explanation: String,
}

#[derive(Serialize)]
pub struct TourStModel {
    pub tour_code: String,
    pub st_code: String,
    pub st_name: String,
    pub explanation: String,
}
#[derive(Serialize)]
pub struct TourThreatIdentificationModel {
    pub tour_code: String,
    pub tour_name: String,
    pub et_list: Vec<TourEtModel>,
    pub st_list: Vec<TourStModel>
}
#[derive(Serialize)]
pub struct EtSummaryModel {
    pub et_code: String,
    pub relevance: i32,
}
#[derive(Serialize)]
pub struct StSummaryModel {
    pub st_code: String,
    pub st_name: String,
}

#[derive(Serialize)]
pub struct ThreatSummaryModel {
    pub et_list: Vec<EtSummaryModel>,
    pub st_list: Vec<StSummaryModel>
}