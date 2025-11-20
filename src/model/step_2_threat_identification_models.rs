use serde::{Deserialize, Serialize};
use crate::enums::step_2_threat_identification_enums::ElementaryThreatRelevance;

#[derive(Deserialize)]
pub struct TourEtReviewModel{
    pub relevance: ElementaryThreatRelevance,
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
pub struct TourThreatIdentification {
    pub et_list: Vec<TourEtModel>
}