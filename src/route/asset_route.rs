use crate::route::asset_route::web::Path;
use actix_web::ResponseError;
use crate::api::ApiResponse;
use actix_web::HttpResponse;
use crate::service::GeneralService;
use actix_web::Responder;
use actix_web::{get, web, Scope};
use actix_web::guard::Patch;
use sqlx::postgres::PgSeverity::Panic;
use crate::configuration::AppState;
use crate::route::GeneralRoute;
use crate::service::asset_service::AssetService;

pub struct AssetRoute;
impl GeneralRoute for AssetRoute {
    fn routes() -> Scope {
        web::scope("/asset")
            .service(asset_list)
            .service(asset_by_code)

    }
}

#[get("/")]
pub async fn asset_list(
    data: web::Data<AppState>
) -> impl Responder {
    match AssetService::list(&data.db).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(err) => err.error_response()
    }
}

#[get("/{asset_code}")]
pub async fn asset_by_code(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let asset_code = path.into_inner();
    match AssetService::get_by_code(&data.db, asset_code).await {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(err) => err.error_response()
    }
}
