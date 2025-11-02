use crate::api::ApiResponse;
use crate::configuration::AppState;
use crate::route::GeneralRoute;
use crate::service::application_service::ApplicationService;
use crate::service::GeneralService;
use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder, ResponseError, Scope};

pub struct ApplicationRoute {}

impl GeneralRoute for ApplicationRoute {
    fn routes() -> Scope {
        web::scope("/application")
            .service(application_list)
            .service(application_get)
    }
}
#[get("/")]
async fn application_list(
    data: web::Data<AppState>
) -> impl Responder {
    match ApplicationService::list(&data.db).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[get("/{code}")]
pub async fn application_get(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    match ApplicationService::get_by_code(&data.db, code).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}