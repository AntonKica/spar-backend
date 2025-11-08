use serde::Deserialize;
use crate::enums::BusinessProcessType;

#[derive(Debug, Clone)]
pub struct TOURElementaryThreatRiskTreatmentModel {
    pub risk_analysis_process_code: String,
    pub asset_code: String,
    pub tour_elementary_threat_code: String,
    pub potential_risk: i32,
    pub remaining_risk: i32,
    pub risk_treatment: i32,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct TOURSpecificThreatRiskTreatmentModel {
    pub risk_analysis_process_code: String,
    pub asset_code: String,
    pub tour_specific_threat_code: String,
    pub potential_risk: i32,
    pub remaining_risk: i32,
    pub risk_treatment: i32,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TOURThreatRiskTreatmentUpdateModel {
    pub potential_risk: i32,
    pub remaining_risk: i32,
    pub risk_treatment: i32,
    pub description: String,
}
