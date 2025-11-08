use actix_web::ResponseError;
use strum::IntoEnumIterator;
use actix_web::{get, web, HttpResponse, Responder, Scope};
use actix_web::web::Path;
use serde::Serialize;
use crate::api::ApiResponse;
use crate::configuration::AppState;
use crate::enums::{ElementaryThreatRelevance, ModuleType, ProtectionNeeds};
use crate::enums::risk_classification_enums::{FrequencyOfOccurrence, PotentialDamage, PotentialRisk};
use crate::enums::risk_treatment_enums::RiskTreatment;
use crate::response::EnumResponse;
use crate::route::GeneralRoute;
use crate::service::ApiError;
use crate::service::it_system_service::ITSystemService;

pub struct EnumRoute {}

impl GeneralRoute for EnumRoute {
    fn routes() -> Scope {
        web::scope("/enum")
            .service(enum_module_type_list)
            .service(enum_protection_needs_list)
            .service(bsi_it_grundschutz_module)
            .service(bsi_it_grundschutz_elementary_threat)
            .service(elmentary_threat_relevance)
            .service(frequency_of_occurrence)
            .service(potential_damage)
            .service(potential_risk)
            .service(risk_treatment)
    }
}

#[get("/module-type/")]
pub async fn enum_module_type_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let data: Vec<EnumResponse> = ModuleType::iter().filter(|mt| !matches!(mt, ModuleType::UNKNOWN)).map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

#[get("/protection-needs/")]
pub async fn enum_protection_needs_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let data: Vec<EnumResponse> = ProtectionNeeds::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

#[derive(Serialize)]
pub struct EnumCodeResponse {
    pub code: String,
    pub name: String,
}
#[get("/bsi-it-grundschutz-module/")]
pub async fn bsi_it_grundschutz_module(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result =  sqlx::query_as!(
        EnumCodeResponse,
        r#"SELECT code, name FROM it_grundschutz_module"#,
    )
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(res) => {
            HttpResponse::Ok().json(ApiResponse::new(res))
        }
        Err(err) => {
            ApiError::Database(err).error_response()
        }
    }
}

#[get("/bsi-it-grundschutz-elementary-threat/")]
pub async fn bsi_it_grundschutz_elementary_threat(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result =  sqlx::query_as!(
        EnumCodeResponse,
        r#"SELECT * FROM it_grundschutz_elementary_threat"#,
    )
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(res) => {
            HttpResponse::Ok().json(ApiResponse::new(res))
        }
        Err(err) => {
            ApiError::Database(err).error_response()
        }
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
pub async fn elmentary_threat_relevance(
    data: web::Data<AppState>,
) -> impl Responder {
    let data: Vec<EnumResponse> = ElementaryThreatRelevance::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

#[get("/risk-treatment/")]
pub async fn risk_treatment(
    data: web::Data<AppState>,
) -> impl Responder {
    let data: Vec<EnumResponse> = RiskTreatment::iter().map(EnumResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}
