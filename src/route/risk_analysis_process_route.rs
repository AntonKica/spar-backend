use crate::route::RiskAnalysisProcessCreateModel;
use crate::api::ApiResponse;
use crate::configuration::AppState;
use crate::route::GeneralRoute;
use crate::service::{ApiError, GeneralService};
use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError, Scope};
use crate::service::risk_analysis_process_service::RiskAnalysisProcessService;

pub struct RiskAnalysisProcessRoute {}

impl GeneralRoute for RiskAnalysisProcessRoute {
    fn routes() -> Scope {
        web::scope("/risk-analysis-process")
            .service(risk_analysis_process_list)
            .service(risk_analysis_process_get)
            .service(risk_analysis_process_create)
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

#[get("/{code}/threat-overview-list")]
pub async fn threat_overview_list(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    match RiskAnalysisProcessService::get_by_code(&data.db, code).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}
