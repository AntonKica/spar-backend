use serde::Serialize;
use crate::enums::asset_enums::{AssetType, ProtectionNeeds};
use crate::model::fulfilled_threat_models::FulfilledThreatDetailModel;
use crate::model::security_measure_models::SecurityMeasureModel;

#[derive(Debug, Clone)]
pub struct AssetCreateModel {
    pub name: String,
    pub asset_type: AssetType,
    pub confidentiality_protection_needs: ProtectionNeeds,
    pub integrity_protection_needs: ProtectionNeeds,
    pub availability_protection_needs: ProtectionNeeds,
    pub description: String,
}
#[derive(Serialize)]
pub struct AssetModel {
    pub code: String,
    pub name: String,
    pub asset_type: i32,
    pub confidentiality_protection_needs: i32,
    pub integrity_protection_needs: i32,
    pub availability_protection_needs: i32,
    pub description: String,
}

#[derive(Serialize)]
pub struct AssetDetailModel {
    pub code: String,
    pub name: String,
    pub asset_type: i32,
    pub confidentiality_protection_needs: i32,
    pub integrity_protection_needs: i32,
    pub availability_protection_needs: i32,
    pub description: String,
    pub fulfilled_threat_list: Vec<FulfilledThreatDetailModel>,
    pub security_measure_list: Vec<SecurityMeasureModel>
}
