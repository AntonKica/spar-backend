use serde::Serialize;

pub struct SecurityMeasureCreateModel {
    pub name: String,
    pub description: String,
    pub confidentiality_protected: bool,
    pub integrity_protected: bool,
    pub availability_protected: bool,
}
#[derive(Serialize, Clone)]
pub struct SecurityMeasureModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub confidentiality_protected: bool,
    pub integrity_protected: bool,
    pub availability_protected: bool,
}