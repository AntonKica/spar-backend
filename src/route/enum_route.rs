use strum::IntoEnumIterator;
use crate::service::ErrorResponse;
use actix_web::web::Json;
use crate::service::ApiResult;
use crate::api::ApiResponse;
/*
use strum::IntoEnumIterator;
use crate::enums::asset_enums::AssetType;
use crate::enums::asset_enums::ProtectionNeeds;
use crate::enums::risk_analysis_process_enums::{ProcessStatus, ProcessStep};
 */
use crate::enums::{EnumCodeName, RiskAnalysisState};
use crate::route::GeneralRoute;
use actix_web::{get, web, HttpResponse, Responder, Scope};
use serde::Serialize;
use sqlx::FromRow;
use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;
/*
use crate::enums::step_2_threat_identification_enums::ThreatRelevance;
use crate::enums::step_3_risk_classification_enums::{ThreatImpact, ThreatProbability, ThreatRisk};
use crate::enums::step_4_risk_treatment_enums::{RiskTransferType, RiskTreatmentType};
 */

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
struct EnumResponse {
    risk_analysis_state: Vec<EnumCodeName>
    /*
    protection_needs: Vec<EnumCodeName>,
    threat_probability: Vec<EnumCodeName>,
    threat_impact: Vec<EnumCodeName>,
    threat_risk: Vec<EnumCodeName>,
    threat_relevance: Vec<EnumCodeName>,
    risk_treatment: Vec<EnumCodeName>,
    asset_type: Vec<EnumCodeName>,
    process_status: Vec<EnumCodeName>,
    process_step: Vec<EnumCodeName>,
    risk_transfer_type: Vec<EnumCodeName>,

     */
}
pub struct EnumRoute {}

impl GeneralRoute for EnumRoute {
    fn configure(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/enum")
                .service(list)
        );
    }
}
#[utoipa::path(
    responses(
        (status = 200, description = "List all enums", body = Vec<EnumResponse>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("")]
async fn list() -> ApiResult<Json<EnumResponse>> {
    Ok(Json(
        EnumResponse {
            risk_analysis_state: RiskAnalysisState::iter().map(EnumCodeName::from).collect(),
            /*
            protection_needs: ProtectionNeeds::iter().map(EnumCodeName::from).collect(),
            threat_probability: ThreatProbability::iter().map(EnumCodeName::from).collect(),
            threat_impact: ThreatImpact::iter().map(EnumCodeName::from).collect(),
            threat_risk: ThreatRisk::iter().map(EnumCodeName::from).collect(),
            threat_relevance: ThreatRelevance::iter().map(EnumCodeName::from).collect(),
            risk_treatment: RiskTreatmentType::iter().map(EnumCodeName::from).collect(),
            asset_type: AssetType::iter().map(EnumCodeName::from).collect(),
            process_status: ProcessStatus::iter().map(EnumCodeName::from).collect(),
            process_step: ProcessStep::iter().map(EnumCodeName::from).collect(),
            risk_transfer_type: RiskTransferType::iter().map(EnumCodeName::from).collect(),
             */
        }
    ))
}
