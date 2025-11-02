use crate::api::ApiResponse;
use crate::configuration::AppState;
use crate::route::GeneralRoute;
use crate::service::it_system_service::ITSystemService;
use crate::service::GeneralService;
use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder, ResponseError, Scope};

pub struct ITSystemRoute {}

impl GeneralRoute for ITSystemRoute {
    fn routes() -> Scope {
        web::scope("/it-system")
            .service(it_system_list)
            .service(it_system_get)
    }
}
#[get("/")]
async fn it_system_list(
    data: web::Data<AppState>
) -> impl Responder {
    match ITSystemService::list(&data.db).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[get("/{code}")]
pub async fn it_system_get(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    match ITSystemService::get_by_code(&data.db, code).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}