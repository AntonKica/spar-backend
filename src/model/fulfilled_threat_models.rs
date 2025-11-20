use serde::Serialize;
use crate::enums::fulfilled_threat_enums::TimeCostUnit;

pub struct FulfilledThreatCreateModel {
    pub et_code: Option<String>,
    pub st_code: Option<String>,
    pub time_cost: Option<i32>,
    pub time_cost_unit: Option<TimeCostUnit>,
    pub monetary_cost: Option<i32>,
    pub description: String,
}
#[derive(Serialize, Clone)]
pub struct FulfilledThreatDetailModel {
    pub code: String,
    pub et_code: Option<String>,
    pub st_code: Option<String>,
    pub threat_name: String,
    pub time_cost: Option<i32>,
    pub time_cost_unit: Option<i32>,
    pub monetary_cost: Option<i32>,
    pub description: Option<String>,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
}