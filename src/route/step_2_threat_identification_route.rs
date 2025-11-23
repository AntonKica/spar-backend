use crate::model::step_2_threat_identification_models::TourThreatReviewModel;
use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError, Scope};
use crate::api::{ApiResponse, ApiSuccessResponse};
use crate::configuration::AppState;
use crate::route::GeneralRoute;
use crate::service::ApiError;
use crate::service::step_2_threat_idenfication_service::Step2ThreatIdentificationService;

pub struct Step2ThreatIdenfiticationRoute {}

impl GeneralRoute for Step2ThreatIdenfiticationRoute {
    fn routes() -> Scope {
        web::scope("/step-2-relevant-threat-identification")
            .service(tour_threat_identification_list)
            .service(threat_identification_list)
            .service(threat_review)
    }
}

#[get("/{rap_code}/")]
async fn tour_threat_identification_list(
    data: web::Data<AppState>,
    path: Path<String>,
) -> impl Responder {
    let rap_code =  path.into_inner();

    match Step2ThreatIdentificationService::list_tour_threat_identification(&data.db, rap_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(e) => e.error_response()
    }
}

#[get("/{rap_code}/{tour_code}/")]
async fn threat_identification_list(
    data: web::Data<AppState>,
    path: Path<(String, String)>,
) -> impl Responder {
    let (rap_code, tour_code) =  path.into_inner();

    match Step2ThreatIdentificationService::list_threat_identification(&data.db, rap_code, tour_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(e) => e.error_response()
    }
}

#[post("/{rap_code}/{tour_code}/review/{et_code}")]
async fn threat_review(
    data: web::Data<AppState>,
    path: Path<(String, String, String)>,
    body: web::Json<TourThreatReviewModel>,
) -> impl Responder {
    let (rap_code, tour_code, threat_code) =  path.into_inner();
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    match Step2ThreatIdentificationService::threat_review(&mut tx, rap_code, tour_code, threat_code, body.0).await {
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