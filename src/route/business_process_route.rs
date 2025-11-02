use actix_web::{get, web, HttpResponse, Responder, ResponseError, Scope};
use actix_web::web::Path;
use serde::Serialize;
use crate::api::ApiResponse;
use crate::configuration::AppState;
use crate::model::BusinessProcessApplicationModel;
use crate::route::{GeneralRoute};
use crate::service::business_process_service::BusinessProcessService;
use crate::service::GeneralService;
use crate::service::role_service::RoleService;

pub struct BusinessProcessRoute {}

impl GeneralRoute for BusinessProcessRoute {
    fn routes() -> Scope {
        web::scope("/business-process")
            .service(business_process_list)
            .service(business_process_get)
            .service(business_process_get_assigned_roles)
            .service(business_process_application_list)
    }
}
#[get("/")]
async fn business_process_list(
    data: web::Data<AppState>
) -> impl Responder {
    match BusinessProcessService::list(&data.db).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[get("/{code}")]
pub async fn business_process_get(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    match BusinessProcessService::get_by_code(&data.db, code).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[get("/{code}/assigned-roles/")]
pub async fn business_process_get_assigned_roles(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    match RoleService::list_for_business_process(&data.db, code).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::new(data)),
        Err(e) => e.error_response()
    }
}

#[derive(Serialize)]
struct BusinessProcessApplicationResponse {
    business_process_code: String,
    application_code: String,
}

impl BusinessProcessApplicationResponse {
    fn filter_db_record(record: BusinessProcessApplicationModel) -> BusinessProcessApplicationResponse {
        BusinessProcessApplicationResponse {
            business_process_code: record.business_process_code,
            application_code: record.application_code,
        }
    }
}

#[get("/application/")]
pub async fn business_process_application_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        BusinessProcessApplicationModel,
        r#"SELECT * FROM business_process__application"#,
    )
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(res) => {
            let data: Vec<BusinessProcessApplicationResponse> = res.into_iter().map(BusinessProcessApplicationResponse::filter_db_record).collect();
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": data }))
        }
        Err(err) => {
            let message = format!("{:?}", err);
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed", "error": message}))
        }
    }
}