pub mod risk_classsification_model;
pub mod risk_treatment_model;
pub mod asset_model;
pub mod fulfilled_threat_models;
pub mod security_measure_models;
pub mod specific_threat_model;
pub mod risk_analysis_process_models;

use serde::{Deserialize, Serialize};
use serde_json::{ Value};

#[derive(Debug, Clone, Deserialize)]
pub struct RiskAnalysisProcessCreateModel {
    pub target_objects_under_review: Vec<String>
}

#[derive(Debug, Clone)]
pub struct TargetObjectUnderReviewCreateModel {
    pub risk_analysis_process_code: String,
    pub asset_code: String,
}

#[derive(Deserialize, Clone)]
pub struct TOURElementaryThreatUpdateModel {
    pub elementary_threat_code: String,
    pub relevance: i32,
    pub comment: String,
    pub reviewed: bool
}

#[derive(Deserialize, Clone)]
pub struct TOURSpecificThreatCreateModel {
    pub name: String,
    pub description: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
}

pub struct TOURSpecificThreatModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
}

pub struct TOURSpecificThreatOverviewModel {
    pub reviewed: bool,
}
