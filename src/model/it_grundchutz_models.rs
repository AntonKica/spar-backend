use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, serde::Serialize, utoipa::ToSchema)]
pub struct ThreatModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
}

#[derive(Debug, Clone, FromRow, serde::Serialize, utoipa::ToSchema)]
pub struct ItGrundschutzModule {
    pub code: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct ItGrundschutzModuleThreat {
    pub it_grundschutz_module: String,
    pub it_grundschutz_threat: String,
}

#[derive(Debug, Clone, FromRow, serde::Serialize, utoipa::ToSchema)]
pub struct ItGrundschutzModuleRequirement {
    pub code: String,
    pub module: String,
    pub description: String,
}