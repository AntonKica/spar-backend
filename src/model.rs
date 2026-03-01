pub mod risk_classsification_model;
pub mod risk_treatment_model;
pub mod asset_model;
pub mod security_measure_models;
pub mod threat_models;
pub mod risk_analysis_process_models;
pub mod step_2_threat_identification_models;
pub mod step_3_risk_classification_models;
pub mod step_4_risk_treatment_models;

use serde::{Deserialize};

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
    pub et_code: String,
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
