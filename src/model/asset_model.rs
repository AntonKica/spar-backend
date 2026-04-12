use crate::model::it_grundchutz_models::ItGrundschutzModule;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, utoipa::ToSchema )]
pub struct AssetModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub module: String,
}

#[derive(Debug, Clone, serde::Deserialize, utoipa::ToSchema)]
pub struct AssetModelCreate {
    pub name: String,
    pub description: String,
    pub module: String,
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct AssetModelDetail {
    pub code: String,
    pub name: String,
    pub description: String,
    pub module: ItGrundschutzModule,
}

/*
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
    pub security_measure_list: Vec<SecurityMeasureModel>
}

*/