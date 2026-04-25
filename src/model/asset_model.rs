use crate::enums::ProtectionRequirement;
use crate::model::it_grundchutz_models::ItGrundschutzModule;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, utoipa::ToSchema)]
pub struct AssetModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub module: String,
    pub confidentiality_protection_requirement: ProtectionRequirement,
    pub integrity_protection_requirement: ProtectionRequirement,
    pub availability_protection_requirement: ProtectionRequirement,
    pub confidentiality_protection_requirement_description: String,
    pub integrity_protection_requirement_description: String,
    pub availability_protection_requirement_description: String,
}

#[derive(Debug, Clone, serde::Deserialize, utoipa::ToSchema)]
pub struct AssetCreateModel {
    pub name: String,
    pub description: String,
    pub module: String,
    pub confidentiality_protection_requirement: ProtectionRequirement,
    pub integrity_protection_requirement: ProtectionRequirement,
    pub availability_protection_requirement: ProtectionRequirement,
    pub confidentiality_protection_requirement_description: String,
    pub integrity_protection_requirement_description: String,
    pub availability_protection_requirement_description: String,
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct AssetDetailModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub module: ItGrundschutzModule,
    pub confidentiality_protection_requirement: ProtectionRequirement,
    pub integrity_protection_requirement: ProtectionRequirement,
    pub availability_protection_requirement: ProtectionRequirement,
    pub confidentiality_protection_requirement_description: String,
    pub integrity_protection_requirement_description: String,
    pub availability_protection_requirement_description: String,
}