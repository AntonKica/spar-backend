use crate::api::ApiResponse;
use strum::IntoEnumIterator;
use crate::enums::asset_enums::AssetType;
use crate::enums::asset_enums::ProtectionNeeds;
use crate::enums::risk_analysis_process_enums::{ProcessStatus, ProcessStep};
use crate::enums::risk_treatment_enums::RiskTreatment;
use crate::enums::EnumCodeName;
use crate::route::GeneralRoute;
use actix_web::{get, web, HttpResponse, Responder, Scope};
use serde::Serialize;
use crate::enums::step_2_threat_identification_enums::ThreatRelevance;
use crate::enums::step_3_risk_classification_enums::{ThreatImpact, ThreatProbability, ThreatRisk};
#[derive(Serialize)]
struct EnumResponse {
    protection_needs: Vec<EnumCodeName>,
    threat_probability: Vec<EnumCodeName>,
    threat_impact: Vec<EnumCodeName>,
    threat_risk: Vec<EnumCodeName>,
    threat_relevance: Vec<EnumCodeName>,
    risk_treatment: Vec<EnumCodeName>,
    asset_type: Vec<EnumCodeName>,
    process_status: Vec<EnumCodeName>,
    process_step: Vec<EnumCodeName>,
}
pub struct EnumRoute {}

impl GeneralRoute for EnumRoute {
    fn routes() -> Scope {
        web::scope("/enum")
            .service(list)
    }
}
#[get("/")]
async fn list() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        EnumResponse {
            protection_needs: ProtectionNeeds::iter().map(EnumCodeName::from).collect(),
            threat_probability: ThreatProbability::iter().map(EnumCodeName::from).collect(),
            threat_impact: ThreatImpact::iter().map(EnumCodeName::from).collect(),
            threat_risk: ThreatRisk::iter().map(EnumCodeName::from).collect(),
            threat_relevance: ThreatRelevance::iter().map(EnumCodeName::from).collect(),
            risk_treatment: RiskTreatment::iter().map(EnumCodeName::from).collect(),
            asset_type: AssetType::iter().map(EnumCodeName::from).collect(),
            process_status: ProcessStatus::iter().map(EnumCodeName::from).collect(),
            process_step: ProcessStep::iter().map(EnumCodeName::from).collect(),
        }
    ))
}
