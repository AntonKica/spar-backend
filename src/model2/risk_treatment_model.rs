use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct TOURElementaryThreatRiskTreatmentModel {
    pub risk_analysis_process_code: String,
    pub asset_code: String,
    pub tour_et_code: String,
    pub potential_risk: i32,
    pub remaining_risk: i32,
    pub risk_treatment: i32,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct TOURSpecificThreatRiskTreatmentModel {
    pub risk_analysis_process_code: String,
    pub asset_code: String,
    pub tour_st_code: String,
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
