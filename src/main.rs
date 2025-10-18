use actix_web::{web, HttpResponse};
use actix_web::Responder;
use actix_web::{get, App, HttpServer};
use actix_web::web::{scope, Path};
use serde::{Serialize};
use sqlx::{Pool, Postgres};
use spar_backend::{create_connection};
use spar_backend::model::{BusinessProcessModel, RoleModel};
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
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}