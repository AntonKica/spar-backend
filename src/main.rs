use actix_web::{web, HttpResponse};
use actix_web::Responder;
use actix_web::{get, App, HttpServer};
use actix_web::web::{scope, Path};
use serde::{Serialize};
use sqlx::{Pool, Postgres};
use strum::IntoEnumIterator;
use spar_backend::{create_connection};
use spar_backend::enums::ModuleType;
use spar_backend::model::{ApplicationModel, BusinessProcessModel, ITSystemCreateModel, RoleModel, ITSystemModel, BusinessProcessApplicationCreateModel, BusinessProcessApplicationModel};
use spar_backend::response::EnumResponse;

#[derive(Serialize)]
struct BusinessProcessResponse {
    code: String,
    name: String,
    description: String,
    process_type: EnumResponse,
    responsible: Option<String>
}

impl BusinessProcessResponse {
    fn filter_db_record(record: BusinessProcessModel) -> BusinessProcessResponse {
        BusinessProcessResponse {
            code: record.code,
            name: record.name,
            description: record.description,
            process_type: EnumResponse::from(record.process_type),
            responsible: record.responsible,
        }
    }
}
#[derive(Serialize)]
struct RoleResponse {
    code: String,
    name: String,
    description: String,
}

impl RoleResponse {
    fn filter_db_record(record: RoleModel) -> RoleResponse {
        RoleResponse {
            code: record.code,
            name: record.name,
            description: record.description,
        }
    }
}

#[derive(Serialize)]
struct ApplicationResponse {
    code: String,
    name: String,
    description: String,
    module_type: EnumResponse,
    application_user: String,
    responsible: String,
}

impl ApplicationResponse {
    fn filter_db_record(record: ApplicationModel) -> ApplicationResponse {
        ApplicationResponse {
            code: record.code,
            name: record.name,
            description: record.description,
            module_type: EnumResponse::from(record.module_type),
            application_user: record.application_user,
            responsible: record.responsible,
        }
    }
}

#[derive(Serialize)]
struct ITSystemResponse {
    code: String,
    name: String,
    description: String,
    count: i32,
    module_type: EnumResponse,
    application_user: String,
    responsible: String,
}

impl ITSystemResponse {
    fn filter_db_record(record: ITSystemModel) -> ITSystemResponse {
        ITSystemResponse {
            code: record.code,
            name: record.name,
            description: record.description,
            count: record.count,
            module_type: EnumResponse::from(record.module_type),
            application_user: record.application_user,
            responsible: record.responsible,
        }
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
#[get("/business-process/")]
async fn business_process_list(
    data: web::Data<AppState>
) -> impl Responder {
    let query_result = sqlx::query_as!(
        BusinessProcessModel,
        r#"SELECT * FROM business_process"#
    )
        .fetch_all(&data.db)
        .await;
    
    match query_result {
        Ok(res) => {
            let data: Vec<BusinessProcessResponse> = res.into_iter().map(BusinessProcessResponse::filter_db_record).collect();
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": data }))
        }
        Err(err) => {
            let message = format!("{:?}", err);
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed", "error": message}))
        }
    }
}

#[get("/business-process/{code}")]
pub async fn business_process_get(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    let query_result =  sqlx::query_as!(
        BusinessProcessModel,
        r#"SELECT * FROM business_process WHERE code = $1"#,
        code
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(bp) => {
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": BusinessProcessResponse::filter_db_record(bp) }))
        }
        Err(err) => {
            let message = format!("{:?}", err);
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed", "error": message}))
        }
    }
}

#[get("/business-process/{code}/assigned-roles/")]
pub async fn business_process_get_assigned_roles(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    let query_result =  sqlx::query_as!(
        RoleModel,
        r#"SELECT * FROM role WHERE EXISTS(
        SELECT * FROM business_process__role WHERE business_process__role.role_code = role.code AND business_process__role.business_process_code = $1 LIMIT 1
        )"#,
        code
    )
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(res) => {
            let data: Vec<RoleResponse> = res.into_iter().map(RoleResponse::filter_db_record).collect();
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": data }))
        }
        Err(err) => {
            let message = format!("{:?}", err);
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed", "error": message}))
        }
    }
}

#[get("/role/")]
pub async fn role_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result =  sqlx::query_as!(
        RoleModel,
        r#"SELECT * FROM role"#,
    )
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(res) => {
            let data: Vec<RoleResponse> = res.into_iter().map(RoleResponse::filter_db_record).collect();
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": data }))
        }
        Err(err) => {
            let message = format!("{:?}", err);
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed", "error": message}))
        }
    }
}
#[get("/role/{code}")]
pub async fn role_get(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    let query_result =  sqlx::query_as!(
        RoleModel,
        r#"SELECT * FROM role WHERE code = $1"#,
        code
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(bp) => {
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": RoleResponse::filter_db_record(bp) }))
        }
        Err(err) => {
            let message = format!("{:?}", err);
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed", "error": message}))
        }
    }
}

#[get("/application/")]
pub async fn application_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result =  sqlx::query_as!(
        ApplicationModel,
        r#"SELECT * FROM application"#,
    )
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(res) => {
            let data: Vec<ApplicationResponse> = res.into_iter().map(ApplicationResponse::filter_db_record).collect();
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": data }))
        }
        Err(err) => {
            let message = format!("{:?}", err);
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed", "error": message}))
        }
    }
}

#[get("/application/{code}")]
pub async fn application_get(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    let query_result =  sqlx::query_as!(
        ApplicationModel,
        r#"SELECT * FROM application"#,
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(res) => {
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": ApplicationResponse::filter_db_record(res) }))
        }
        Err(err) => {
            let message = format!("{:?}", err);
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed", "error": message}))
        }
    }
}

#[get("/it-system/")]
pub async fn it_system_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result =  sqlx::query_as!(
        ITSystemModel,
        r#"SELECT * FROM it_system"#,
    )
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(res) => {
            let data: Vec<ITSystemResponse> = res.into_iter().map(ITSystemResponse::filter_db_record).collect();
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": data }))
        }
        Err(err) => {
            let message = format!("{:?}", err);
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed", "error": message}))
        }
    }
}

#[get("/it-system/{code}")]
pub async fn it_system_get(
    data: web::Data<AppState>,
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    let query_result =  sqlx::query_as!(
        ITSystemModel,
        r#"SELECT * FROM it_system"#,
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(res) => {
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": ITSystemResponse::filter_db_record(res) }))
        }
        Err(err) => {
            let message = format!("{:?}", err);
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed", "error": message}))
        }
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

pub struct AppState {
    db: Pool<Postgres>,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = create_connection().await;
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState{ db: db.clone() }))
            .service(
                scope("/svc")
                    .service(business_process_list)
                    .service(business_process_get)
                    .service(business_process_get_assigned_roles)
                    .service(role_list)
                    .service(role_get)
                    .service(application_list)
                    .service(application_get)
                    .service(it_system_list)
                    .service(it_system_get)
                    .service(enum_module_type_list)
                    .service(bsi_it_grundschutz_module)
                    .service(bsi_it_grundschutz_elementary_threat)
                    .service(business_process_application_list)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}