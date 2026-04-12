use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ThreatCreateModel {
    pub name: String,
    pub description: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
}
#[derive(Serialize)]
pub struct ThreatModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
}
