use chrono::NaiveDate;
use serde::Serialize;
use crate::model::asset_model::AssetModel;

#[derive(Serialize)]
pub struct CodeNameModel {
    pub code: String,
    pub name: String,
}
#[derive(Serialize)]
pub struct RiskAnalysisProcessModel{
    pub code: String,
    pub created_on: NaiveDate,
    pub process_status: i32,
    pub process_step: i32,
}

#[derive(Serialize)]
pub struct RiskAnalysisProcessDetailModel{
    pub code: String,
    pub created_on: NaiveDate,
    pub process_status: i32,
    pub process_step: i32,
    pub tour_list: Vec<AssetModel>,
}