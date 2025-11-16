use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError, Scope};
use crate::api::{ApiCodeResponse, ApiResponse, ApiSuccessResponse};
use crate::configuration::AppState;
use crate::route::GeneralRoute;
use crate::service::ApiError;
use crate::service::risk_analysis_process_service::RiskAnalysisProcessService;

pub struct RiskAnalysisProcessRoute {}

impl GeneralRoute for RiskAnalysisProcessRoute {
    fn routes() -> Scope {
        web::scope("/risk-analysis-process")
            .service(create)
            .service(get)
            .service(set_tour)
    }
}

#[post("")]
async fn create(
    data: web::Data<AppState>,
) -> impl Responder {
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    match RiskAnalysisProcessService::create(&mut tx).await {
        Ok(code) => {
            if let Err(e) = tx.commit().await {
                return ApiError::Database(e).error_response();
            }
            HttpResponse::Ok().json(ApiCodeResponse::new(code))
        }
        Err(e) => {
            let _ = tx.rollback().await;
            e.error_response()
        }
    }
}

#[get("/{rap_code}")]
async fn get(
    data: web::Data<AppState>,
    path: Path<String>,
) -> impl Responder {
    let rap_code = path.into_inner();
    match RiskAnalysisProcessService::get_by_code(&data.db, rap_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(e) => e.error_response()

    }
}

#[post("/{rap_code}/tour")]
async fn set_tour(
    data: web::Data<AppState>,
    path: Path<String>,
    body: web::Json<Vec<String>>,
) -> impl Responder {
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    let rap_code = path.into_inner();
    match RiskAnalysisProcessService::set_tour(&mut *tx, rap_code, body.0).await {
        Ok(code) => {
            if let Err(e) = tx.commit().await {
                return ApiError::Database(e).error_response();
            }
            HttpResponse::Ok().json(ApiSuccessResponse::new())
        }
        Err(e) => {
            let _ = tx.rollback().await;
            e.error_response()
        }
    }
}
