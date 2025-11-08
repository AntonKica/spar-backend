use serde::Deserialize;
use crate::enums::BusinessProcessType;

#[derive(Debug, Clone)]
pub struct TOURElementaryThreatRiskClassificationModel {
    pub risk_analysis_process_code: String,
    pub asset_code: String,
    pub tour_elementary_threat_code: String,
    pub frequency_of_occurrence: i32,
    pub potential_damage: i32,
    pub description: String,
    pub evaluation: String,
}

#[derive(Debug, Clone)]
pub struct TOURSpecificThreatRiskClassificationModel {
    pub risk_analysis_process_code: String,
    pub asset_code: String,
    pub tour_specific_threat_code: String,
    pub frequency_of_occurrence: i32,
    pub potential_damage: i32,
    pub description: String,
    pub evaluation: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TOURElementaryThreatRiskClassificationUpdateModel {
    pub frequency_of_occurrence: i32,
    pub potential_damage: i32,
    pub description: String,
    pub evaluation: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TOURSpecificThreatRiskClassificationUpdateModel {
    pub frequency_of_occurrence: i32,
    pub potential_damage: i32,
    pub description: String,
    pub evaluation: String,
}
