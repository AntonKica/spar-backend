use crate::service::ApiError;
use actix_web::web::Path;
use actix_web::{post, ResponseError};
use crate::api::{ApiCodeResponse, ApiResponse};
use actix_web::HttpResponse;
use crate::service::GeneralService;
use actix_web::Responder;
use actix_web::{get, web, Scope};
use crate::configuration::AppState;
use crate::model::threat_models::ThreatCreateModel;
use crate::route::GeneralRoute;
use crate::service::threat_service::ThreatService;

pub struct SpecificThreatRoute;
impl GeneralRoute for SpecificThreatRoute {
    fn routes() -> Scope {
        web::scope("/threat")
            .service(list)
            .service(by_code)
            .service(create)

    }
}

#[post("")]
async fn create(
    data: web::Data<AppState>,
    body: web::Json<ThreatCreateModel>
) -> impl Responder {
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiError::Database(e).error_response(),
    };

    match ThreatService::create(&mut *tx, body.0).await {
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

#[get("/")]
async fn list(
    data: web::Data<AppState>
) -> impl Responder {
    match ThreatService::list(&data.db).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(err) => err.error_response()
    }
}

#[get("/{code}")]
async fn by_code(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let threat_code = path.into_inner();
    match ThreatService::get_by_code(&data.db, threat_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(err) => err.error_response()
    }
}

