use crate::service::ApiError;
use crate::model::step_3_risk_classification_models::TourRiskClassificationClassifyModel;
use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError, Scope};
use crate::api::{ApiResponse, ApiSuccessResponse};
use crate::configuration::AppState;
use crate::route::GeneralRoute;
use crate::service::step_3_risk_classification_service::Step3RiskClassificationService;

pub struct Step3RiskClassificationRoute {}

impl GeneralRoute for Step3RiskClassificationRoute {
    fn routes() -> Scope {
        web::scope("/step-3-risk-classification")
            .service(risk_classification_summary)
            .service(tour_risk_classification_list)
            .service(threat_classify)
    }
}

#[get("/{rap_code}")]
async fn risk_classification_summary(
    data: web::Data<AppState>,
    path: Path<String>,
) -> impl Responder {
    let rap_code =  path.into_inner();

    match Step3RiskClassificationService::risk_classification_summary(&data.db, rap_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(e) => e.error_response()
    }
}

#[get("/{rap_code}/{tour_code}/")]
async fn tour_risk_classification_list(
    data: web::Data<AppState>,
    path: Path<(String, String)>,
) -> impl Responder {
    let (rap_code, tour_code) =  path.into_inner();

    match Step3RiskClassificationService::tour_risk_classification_list(&data.db, rap_code, tour_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(e) => e.error_response()
    }
}

#[post("/{rap_code}/{tour_code}/classify/{threat_code}")]
async fn threat_classify(
    data: web::Data<AppState>,
    path: Path<(String, String, String)>,
    body: web::Json<TourRiskClassificationClassifyModel>,
) -> impl Responder {
    let (rap_code, tour_code, threat_code) =  path.into_inner();
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    match Step3RiskClassificationService::threat_classify(&mut tx, rap_code, tour_code, threat_code, body.0).await {
        Ok(res) => {
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