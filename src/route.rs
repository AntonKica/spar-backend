pub mod business_process_route;
pub mod application_route;
pub mod role_route;
pub mod it_system_route;
pub mod risk_analysis_process_route;
pub mod enum_route;

use crate::model::{AssetModel};
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError, Scope};
use actix_web::web::Path;
use serde::{Serialize};
use strum::IntoEnumIterator;
use crate::api::{ApiResponse};
use crate::configuration::AppState;
use crate::enums::{ModuleType, ElementaryThreatRelevance, ProtectionNeeds};
use crate::model::{RiskAnalysisProcessCreateModel};
use crate::response::{EnumResponse};
use crate::service::application_service::ApplicationService;
use crate::service::it_system_service::ITSystemService;
use crate::service::role_service::RoleService;
use crate::service::{ApiError, GeneralService};


pub trait GeneralRoute {
    fn routes() -> actix_web::Scope;
}

pub struct OtherRotes {}

impl GeneralRoute for OtherRotes {
    fn routes() -> Scope {
        web::scope("")
            .service(asset_list)
    }
}
#[get("/")]
pub async fn role_list(
    data: web::Data<AppState>,
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

#[get("/")]
pub async fn application_list(
    data: web::Data<AppState>,
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

#[get("/")]
pub async fn it_system_list(
    data: web::Data<AppState>,
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
#[get("/asset/")]
pub async fn asset_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        AssetModel,
        r#"SELECT * FROM asset"#,
    )
        .fetch_all(&data.db)
        .await;
    match query_result {
        Ok(res) => {
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": res }))
        }
        Err(err) => {
            let message = format!("{:?}", err);
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed", "error": message}))
        }
    }
}