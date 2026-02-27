use chrono::NaiveDate;
use serde::Serialize;


#[derive(Serialize)]
pub struct RiskAnalysisProcessResponse {
    pub code: String,
    pub created_on: NaiveDate,
}