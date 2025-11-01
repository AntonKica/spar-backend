use crate::model::{AssetModel, Creatable};
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};
use actix_web::web::Path;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use strum::IntoEnumIterator;
use crate::configuration::AppState;
use crate::enums::ModuleType;
use crate::model::{BusinessProcessApplicationModel, RiskAnalysisProcessCreateModel};
use crate::response::EnumResponse;
use crate::service::application_service::ApplicationService;
use crate::service::business_process_service::BusinessProcessService;
use crate::service::it_system_service::ITSystemService;
use crate::service::role_service::RoleService;
use crate::service::service::GeneralService;

#[derive(Serialize)]
struct ApiResponse<T> {
    pub status: &'static str,
    pub data: T,
}

#[derive(Serialize)]
struct ApiErrorResponse {
    pub status: &'static str,
    pub message: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            status: "ok",
            data,
        }
    }
}

impl ApiErrorResponse {
    pub fn new(message: String) -> Self {
        Self {
            status: "error",
            message,
        }
    }
}

fn to_error_response<E: std::fmt::Display>(error: E) -> ApiErrorResponse {
    ApiErrorResponse::new(error.to_string())
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

#[get("/enum/module-type/")]
pub async fn enum_module_type_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let data: Vec<EnumResponse> = ModuleType::iter().filter(|mt| !matches!(mt, ModuleType::UNKNOWN)).map(EnumResponse::from).collect();
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": data }))
}

#[derive(Serialize)]
pub struct EnumCodeResponse {
    pub code: String,
    pub name: String,
}
#[get("/enum/bsi-it-grundschutz-module/")]
pub async fn bsi_it_grundschutz_module(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result =  sqlx::query_as!(
        EnumCodeResponse,
        r#"SELECT * FROM it_grundschutz_module"#,
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

#[get("/enum/bsi-it-grundschutz-elementary-threat/")]
pub async fn bsi_it_grundschutz_elementary_threat(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result =  sqlx::query_as!(
        EnumCodeResponse,
        r#"SELECT * FROM it_grundschutz_elementary_threat"#,
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

#[get("/business-process-application/")]
pub async fn business_process_application_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result =  sqlx::query_as!(
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

#[derive(Deserialize)]
pub struct RiskAnalysisPostCreateModel {
    pub target_objects_under_review: Vec<String>
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
#[post("/risk-analysis-process/create")]
pub async fn risk_analysis_process_create(
    body: web::Json<RiskAnalysisPostCreateModel>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut tx = match data.db.begin().await {
        Ok(tx) => { tx }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::Ok().json(serde_json::json!({"status": "error","message": message}));
        }
    };

    let create_res = RiskAnalysisProcessCreateModel { created_on: Utc::now().date_naive(), }.create(&mut tx).await;
    let risk_analysis_process_code = match create_res  {
        Ok(res) => {
            res
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::Ok().json(serde_json::json!({"status": "error","message": message}));
        }
    };

    let target_objects_under_review = &body.target_objects_under_review;
    let query = r"
            INSERT INTO target_object_under_review (
                risk_analysis_process_code,
                asset_code
            ) SELECT * FROM UNNEST(
                $1::CHAR(8)[],
                $2::VARCHAR(20)[]
            ) ON CONFLICT DO NOTHING";

    let res = sqlx::query(query)
        .bind(vec![risk_analysis_process_code.to_owned(); target_objects_under_review.len()])
        .bind(target_objects_under_review)
        .fetch_all(&mut *tx)
        .await;

    match res  {
        Ok(res) => {}
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::Ok().json(serde_json::json!({"status": "error","message": message}));
        }
    };

    match tx.commit().await {
        Ok(_) => { HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "code": risk_analysis_process_code })) }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            HttpResponse::Ok().json(serde_json::json!({"status": "error","message": message}))
        }
    }
}


pub fn business_process_routes() -> actix_web::Scope {
    web::scope("/business-process")
        .service(business_process_application_list)
        .service(business_process_get)
        .service(business_process_get_assigned_roles)
}

pub fn role_routes() -> actix_web::Scope {
    web::scope("/role")
        .service(role_list)
        .service(role_get)
}
pub fn application_routes() -> actix_web::Scope {
    web::scope("/application")
        .service(application_list)
        .service(application_get)
}

pub fn it_system_routes() -> actix_web::Scope {
    web::scope("/it_system")
        .service(it_system_list)
        .service(it_system_get)
}
pub fn other_routes() -> actix_web::Scope {
    web::scope("")
        .service(enum_module_type_list)
        .service(bsi_it_grundschutz_module)
        .service(bsi_it_grundschutz_elementary_threat)
        .service(business_process_application_list)
        .service(asset_list)
        .service(risk_analysis_process_create)
}
