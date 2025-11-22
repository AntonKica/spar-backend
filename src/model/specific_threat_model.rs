use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SpecificThreatCreateModel {
    pub name: String,
    pub description: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
}
#[derive(Serialize)]
pub struct SpecificThreatModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
}
