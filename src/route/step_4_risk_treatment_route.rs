use crate::model::step_4_risk_treatment_models::RiskAvoidanceCreateModel;
use crate::service::ApiError;
use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError, Scope};
use crate::api::{ApiResponse, ApiSuccessResponse};
use crate::configuration::AppState;
use crate::enums::risk_analysis_process_enums::ProcessStep::Step4RiskTreatment;
use crate::model::step_4_risk_treatment_models::RiskAcceptanceCreateModel;
use crate::route::GeneralRoute;
use crate::service::step_4_risk_treatment_service::Step4RiskTreatmentService;

pub struct Step4RiskTreatmentRoute {}

impl GeneralRoute for Step4RiskTreatmentRoute {
    fn routes() -> Scope {
        web::scope("/step-4-risk-treatment")
            .service(risk_classification)
            .service(risk_treatment)

            .service(list_acceptance)
            .service(risk_acceptance_by_code)
            .service(risk_accept)
            .service(risk_accept_with_create)

            .service(list_avoidance)
            .service(risk_avoidance_by_code)
            .service(risk_avoid)
            .service(risk_avoid_with_create)
    }
}

#[get("/{rap_code}/{tour_code}/")]
async fn risk_classification(
    data: web::Data<AppState>,
    path: Path<(String, String)>,
) -> impl Responder {
    let (rap_code, tour_code) =  path.into_inner();

    match Step4RiskTreatmentService::tour_risk_classification_list(&data.db, rap_code, tour_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(e) => e.error_response()
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


#[get("/risk-acceptance/")]
async fn list_acceptance(
    data: web::Data<AppState>,
) -> impl Responder {
    match Step4RiskTreatmentService::list_risk_acceptance(&data.db).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(err) => err.error_response()
    }
}

#[get("/risk-acceptance/{acp_code}")]
async fn risk_acceptance_by_code(
    data: web::Data<AppState>,
    path: Path<String>,
) -> impl Responder {
    let acp_code =  path.into_inner();

    match Step4RiskTreatmentService::risk_acceptance_by_code(&data.db, acp_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(e) => e.error_response()
    }
}


#[post("/{rap_code}/{tour_code}/{threat_code}/risk-accept/{acp_code}")]
async fn risk_accept(
    data: web::Data<AppState>,
    path: Path<(String, String, String, String)>,
) -> impl Responder {
    let (rap_code, tour_code, threat_code, acp_code) =  path.into_inner();
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    match Step4RiskTreatmentService::risk_accept(&mut tx, rap_code, tour_code, threat_code, acp_code).await {
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

#[post("/{rap_code}/{tour_code}/{threat_code}/risk-accept")]
async fn risk_accept_with_create(
    data: web::Data<AppState>,
    path: Path<(String, String, String)>,
    body: web::Json<RiskAcceptanceCreateModel>,
) -> impl Responder {
    let (rap_code, tour_code, threat_code) =  path.into_inner();
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    match Step4RiskTreatmentService::risk_accept_with_create(&mut tx, rap_code, tour_code, threat_code, body.0).await {
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

#[get("/risk-avoidance/")]
async fn list_avoidance(
    data: web::Data<AppState>,
) -> impl Responder {
    match Step4RiskTreatmentService::list_risk_avoidance(&data.db).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(err) => err.error_response()
    }
}

#[get("/risk-avoidance/{avd_code}")]
async fn risk_avoidance_by_code(
    data: web::Data<AppState>,
    path: Path<String>,
) -> impl Responder {
    let avd_code =  path.into_inner();

    match Step4RiskTreatmentService::risk_avoidance_by_code(&data.db, avd_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(e) => e.error_response()
    }
}

#[post("/{rap_code}/{tour_code}/{threat_code}/risk-avoid/{avd_code}")]
async fn risk_avoid(
    data: web::Data<AppState>,
    path: Path<(String, String, String, String)>,
) -> impl Responder {
    let (rap_code, tour_code, threat_code, avd_code) =  path.into_inner();
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    match Step4RiskTreatmentService::risk_avoid(&mut tx, rap_code, tour_code, threat_code, avd_code).await {
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

#[post("/{rap_code}/{tour_code}/{threat_code}/risk-avoid")]
async fn risk_avoid_with_create(
    data: web::Data<AppState>,
    path: Path<(String, String, String)>,
    body: web::Json<RiskAvoidanceCreateModel>,
) -> impl Responder {
    let (rap_code, tour_code, threat_code) =  path.into_inner();
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    match Step4RiskTreatmentService::risk_avoid_with_create(&mut tx, rap_code, tour_code, threat_code, body.0).await {
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
