use crate::model::risk_classsification_model::TOURSpecificThreatRiskClassificationUpdateModel;
use crate::model::risk_classsification_model::TOURElementaryThreatRiskClassificationUpdateModel;
use crate::model::{TOURElementaryThreatUpdateModel, TOURSpecificThreatCreateModel};
use crate::route::RiskAnalysisProcessCreateModel;
use crate::api::ApiResponse;
use crate::configuration::AppState;
use crate::route::GeneralRoute;
use crate::service::{ApiError, GeneralService};
use actix_web::web::Path;
use actix_web::{get, post, delete, web, HttpResponse, Responder, ResponseError, Scope};
use crate::service::risk_analysis_process_service::RiskAnalysisProcessService;
use crate::service::risk_classification_service::RiskClassificationService;

pub struct RiskAnalysisProcessRoute {}

impl GeneralRoute for RiskAnalysisProcessRoute {
    fn routes() -> Scope {
        web::scope("/risk-analysis-process")
            .service(risk_analysis_process_list)
            .service(risk_analysis_process_get)
            .service(risk_analysis_process_create)
            .service(threat_overview_list)
            .service(elementary_threat_list)
            .service(elementary_threat_list_update)
            .service(specific_threat_list)
            .service(specific_threat_create)
            .service(specific_threat_update)
            .service(specific_threat_delete)
            .service(specific_threat_overview)
            .service(specific_threat_overview_set_reviewed)
            .service(step_1_threat_overview_finish)
            .service(risk_classification_list)
            .service(update_risk_classification_elementary_threat)
            .service(update_risk_classification_specific_threat)

    }
}

#[get("/")]
pub async fn risk_analysis_process_list(
    data: web::Data<AppState>,
) -> impl Responder {
    match RiskAnalysisProcessService::list(&data.db).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[get("/{code}")]
pub async fn risk_analysis_process_get(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    match RiskAnalysisProcessService::get_by_code(&data.db, code).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[post("/create")]
pub async fn risk_analysis_process_create(
    body: web::Json<RiskAnalysisProcessCreateModel>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    match RiskAnalysisProcessService::create(&mut tx, body.into_inner()).await {
        Ok(code) => {
            if let Err(e) = tx.commit().await {
                return ApiError::Database(e).error_response();
            }
            HttpResponse::Ok().json(ApiResponse::new(serde_json::json!({ "code": code })))
        }
        Err(e) => {
            let _ = tx.rollback().await;
            e.error_response()
        }
    }
}

#[get("/{code}/threat-overview/")]
pub async fn threat_overview_list(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    match RiskAnalysisProcessService::get_threat_overview(&data.db, code).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[get("/{code}/elementary-threat/{asset}")]
pub async fn elementary_threat_list(
    data: web::Data<AppState>,
    path: Path<(String, String)>
) -> impl Responder {
    let (code, asset) = path.into_inner();
    match RiskAnalysisProcessService::get_elementary_threat_list(&data.db, code, asset).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[post("/{code}/elementary-threat/{asset}")]
pub async fn elementary_threat_list_update(
    data: web::Data<AppState>,
    body: web::Json<Vec<TOURElementaryThreatUpdateModel>>,
    path: Path<(String, String)>
) -> impl Responder {
    let (code, asset) = path.into_inner();
    match RiskAnalysisProcessService::update_elementary_threat_list(&data.db, code, asset, body.0).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[get("/{code}/specific-threat/{asset}/")]
pub async fn specific_threat_list(
    data: web::Data<AppState>,
    path: Path<(String, String)>
) -> impl Responder {
    let (code, asset) = path.into_inner();
    match RiskAnalysisProcessService::get_specific_threat_list(&data.db, code, asset).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[post("/{code}/specific-threat/{asset}")]
pub async fn specific_threat_create(
    data: web::Data<AppState>,
    body: web::Json<TOURSpecificThreatCreateModel>,
    path: Path<(String, String)>
) -> impl Responder {
    let (code, asset) = path.into_inner();
    match RiskAnalysisProcessService::create_specific_threat(&data.db, code, asset, body.0).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[post("/{code}/specific-threat/{asset}/{threat}")]
pub async fn specific_threat_update(
    data: web::Data<AppState>,
    body: web::Json<TOURSpecificThreatCreateModel>,
    path: Path<(String, String, String)>
) -> impl Responder {
    let (code, asset, threat) = path.into_inner();
    match RiskAnalysisProcessService::update_specific_threat(&data.db, code, asset, threat, body.0).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}
#[delete("/{code}/specific-threat/{asset}/{threat}")]
pub async fn specific_threat_delete(
    data: web::Data<AppState>,
    path: Path<(String, String, String)>
) -> impl Responder {
    let (code, asset, threat) = path.into_inner();
    match RiskAnalysisProcessService::delete_specific_threat(&data.db, code, asset, threat).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[get("/{code}/specific-threat/{asset}/overview")]
pub async fn specific_threat_overview(
    data: web::Data<AppState>,
    path: Path<(String, String)>
) -> impl Responder {
    let (code, asset) = path.into_inner();
    match RiskAnalysisProcessService::get_specific_threat_overview(&data.db, code, asset).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}
#[post("/{code}/specific-threat/{asset}/overview/reviewed/{value}")]
pub async fn specific_threat_overview_set_reviewed(
    data: web::Data<AppState>,
    path: Path<(String, String, bool)>
) -> impl Responder {
    let (code, asset, reviwed) = path.into_inner();
    match RiskAnalysisProcessService::specific_threat_overview_set_reviewed(&data.db, code, asset, reviwed).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[post("/{rap}/step-1-threat-overview/finish")]
pub async fn step_1_threat_overview_finish(
    data: web::Data<AppState>,
    path: Path<(String)>
) -> impl Responder {
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    let rap = path.into_inner();
    match RiskAnalysisProcessService::step_1_threat_overview_finish(&mut tx, rap).await {
        Ok(res) => {
            if let Err(e) = tx.commit().await {
                return ApiError::Database(e).error_response();
            }
            HttpResponse::Ok().json(ApiResponse::new({}))
        }
        Err(e) => {
            let _ = tx.rollback().await;
            e.error_response()
        }
    }
}

#[get("/{code}/risk-classification/{asset}/")]
pub async fn risk_classification_list(
    data: web::Data<AppState>,
    path: Path<(String, String)>
) -> impl Responder {
    let (code, asset) = path.into_inner();
    match RiskClassificationService::get_risk_classification_list(&data.db, code, asset).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[post("/{rap}/risk-classification/{asset}/elementary-threat/{threat}")]
pub async fn update_risk_classification_elementary_threat(
    data: web::Data<AppState>,
    body: web::Json<TOURElementaryThreatRiskClassificationUpdateModel>,
    path: Path<(String, String, String)>
) -> impl Responder {
    let (rap, asset, threat) = path.into_inner();
    match RiskClassificationService::update_risk_classification_elementary_threat(&data.db, rap, asset, threat, body.0).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[post("/{rap}/risk-classification/{asset}/specific-threat/{threat}")]
pub async fn update_risk_classification_specific_threat(
    data: web::Data<AppState>,
    body: web::Json<TOURSpecificThreatRiskClassificationUpdateModel>,
    path: Path<(String, String, String)>
) -> impl Responder {
    let (rap, asset, threat) = path.into_inner();
    match RiskClassificationService::update_risk_classification_specific_threat(&data.db, rap, asset, threat, body.0).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}
