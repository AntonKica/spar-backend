use crate::api::ApiResponse;
use crate::configuration::AppState;
use crate::route::GeneralRoute;
use crate::service::role_service::RoleService;
use crate::service::GeneralService;
use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder, ResponseError, Scope};

pub struct RoleRoute {}

impl GeneralRoute for RoleRoute {
    fn routes() -> Scope {
        web::scope("/role")
            .service(role_list)
            .service(role_get)
    }
}
#[get("/")]
async fn role_list(
    data: web::Data<AppState>
) -> impl Responder {
    match RoleService::list(&data.db).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[get("/{code}")]
pub async fn role_get(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    match RoleService::get_by_code(&data.db, code).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}