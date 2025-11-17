use crate::enums::asset_enums::ProtectionNeeds;
use actix_web::ResponseError;
use strum::IntoEnumIterator;
use actix_web::{get, web, HttpResponse, Responder, Scope};
use actix_web::web::Path;
use serde::Serialize;
use crate::api::ApiResponse;
use crate::configuration::AppState;
use crate::enums::{ElementaryThreatRelevance, ModuleType};
use crate::enums::asset_enums::AssetType;
use crate::enums::risk_analysis_process_enums::{ProcessStatus, ProcessStep};
use crate::enums::risk_classification_enums::{FrequencyOfOccurrence, PotentialDamage, PotentialRisk};
use crate::enums::risk_treatment_enums::RiskTreatment;
use crate::response::EnumResponse;
use crate::route::GeneralRoute;
use crate::service::ApiError;

pub struct EnumRoute {}

impl GeneralRoute for EnumRoute {
    fn routes() -> Scope {
        web::scope("/enum")
            .service(enum_protection_needs_list)
            .service(elementary_threat_list)
            .service(elmentary_threat_relevance)
            .service(frequency_of_occurrence)
            .service(potential_damage)
            .service(potential_risk)
            .service(risk_treatment)
            .service(asset_type_list)
            .service(process_stauts)
            .service(process_step)
    }
}

#[get("/protection-needs/")]
async fn enum_protection_needs_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let data: Vec<EnumResponse> = ProtectionNeeds::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

#[derive(Serialize)]
struct EnumCodeResponse {
    code: String,
    name: String,
}
#[get("/elementary-threat/")]
async fn elementary_threat_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(EnumCodeResponse, r#"SELECT code, name FROM elementary_threat"#) .fetch_all(&data.db) .await;

    match query_result {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(err) => ApiError::Database(err).error_response()
    }
}

#[get("/frequency-of-occurrence/")]
async fn frequency_of_occurrence() -> impl Responder {
    let data: Vec<EnumResponse> = FrequencyOfOccurrence::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

#[get("/potential-damage/")]
async fn potential_damage() -> impl Responder {
    let data: Vec<EnumResponse> = PotentialDamage::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

#[get("/potential-risk/")]
async fn potential_risk() -> impl Responder {
    let data: Vec<EnumResponse> = PotentialRisk::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

#[get("/elementary-threat-relevance/")]
async fn elmentary_threat_relevance(
    data: web::Data<AppState>,
) -> impl Responder {
    let data: Vec<EnumResponse> = ElementaryThreatRelevance::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

#[get("/risk-treatment/")]
async fn risk_treatment(
    data: web::Data<AppState>,
) -> impl Responder {
    let data: Vec<EnumResponse> = RiskTreatment::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

#[get("/asset-type/")]
async fn asset_type_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let data: Vec<EnumResponse> = AssetType::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

#[get("/process-status/")]
async fn process_stauts(
    data: web::Data<AppState>,
) -> impl Responder {
    let data: Vec<EnumResponse> = ProcessStatus::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

#[get("/process-step/")]
async fn process_step(
    data: web::Data<AppState>,
) -> impl Responder {
    let data: Vec<EnumResponse> = ProcessStep::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}
