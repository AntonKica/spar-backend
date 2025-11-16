use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct RiskAnalysisProcessModel{
    pub code: String,
    pub created_on: NaiveDate,

    pub process_status: i32,
    pub step_1_select_tour_process_status: i32,
    pub step_2_threat_identification_process_status: i32,
    pub step_3_risk_analysis_process_status: i32,
    pub step_4_risk_treatment_process_status: i32,
    pub step_5_risk_treatment_check_process_status: i32,

    pub tour_list: Vec<String>,
}