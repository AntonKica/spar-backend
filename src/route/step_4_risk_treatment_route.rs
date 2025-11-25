use crate::service::ApiError;
use crate::model::step_3_risk_classification_models::TourRiskClassificationClassifyModel;
use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError, Scope};
use crate::api::{ApiResponse, ApiSuccessResponse};
use crate::configuration::AppState;
use crate::route::GeneralRoute;
use crate::service::step_4_risk_treatment_service::Step4RiskTreatmentService;

pub struct Step4RiskTreatmentRoute {}

impl GeneralRoute for Step4RiskTreatmentRoute {
    fn routes() -> Scope {
        web::scope("/step-4-risk-treatment")
            .service(risk_treatment)
            .service(risk_acceptance)
            .service(risk_accept)
    }
}

#[get("/{rap_code}/{tour_code}/{threat_code}")]
async fn risk_treatment(
    data: web::Data<AppState>,
    path: Path<(String, String, String)>,
) -> impl Responder {
    let (rap_code, tour_code, threat_code) =  path.into_inner();

    match Step4RiskTreatmentService::risk_treatment(&data.db, rap_code, tour_code, threat_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(e) => e.error_response()
    }
}

#[get("/{rap_code}/{tour_code}/{threat_code}/acceptance")]
async fn risk_acceptance(
    data: web::Data<AppState>,
    path: Path<(String, String, String)>,
) -> impl Responder {
    let (rap_code, tour_code, threat_code) =  path.into_inner();

    match Step4RiskTreatmentService::risk_acceptance(&data.db, rap_code, tour_code, threat_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(e) => e.error_response()
    }
}

#[post("/{rap_code}/{tour_code}/{threat_code}/accept")]
async fn risk_accept(
    data: web::Data<AppState>,
    path: Path<(String, String, String)>,
    body: web::Json<()>,
) -> impl Responder {
    let (rap_code, tour_code, threat_code) =  path.into_inner();
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    match Step4RiskTreatmentService::risk_accept(&mut tx, rap_code, tour_code, threat_code, body.0).await {
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