use actix_web::{web, HttpResponse};
use actix_web::Responder;
use actix_web::{get, App, HttpServer};
use actix_web::web::{scope, Path};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use spar_backend::create_connection;
use spar_backend::BusinessProcessModel;

enum ProcessType {
    SUPPORT,
    CORE
}
#[derive(Serialize)]
struct BusinessProcessResponse {
    code: String,
    name: String,
    description: String,
    process_type: String,
    responsible: String
}

impl BusinessProcessResponse {
    fn filter_db_record(record: BusinessProcessModel) -> BusinessProcessResponse {
        BusinessProcessResponse {
            code: record.code,
            name: record.name,
            description: record.description,
            process_type: record.process_type,
            responsible: record.responsible,
        }
    }
}

#[get("/business-process")]
async fn business_process_list(
    data: web::Data<AppState>
) -> impl Responder {
    let business_processes: Vec<BusinessProcessModel> = sqlx::query_as!(
        BusinessProcessModel,
        r#"SELECT * FROM business_process"#
    )
        .fetch_all(&data.db)
        .await
        .unwrap();
    
    let data: Vec<BusinessProcessResponse> = business_processes.into_iter().map(BusinessProcessResponse::filter_db_record).collect();
    HttpResponse::Ok().json(serde_json::json!(data))
}

#[get("/business-process/{code}")]
pub async fn business_process_get(
    path: Path<String>
) -> impl Responder {
    let code = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({ "status": "failed" }))
    /*
    match list_business_processes().iter().find(|bp| { bp.code == code }) {
        Some(bp) => {
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "data": bp }))
        }
        None => {
            HttpResponse::Ok().json(serde_json::json!({ "status": "failed" }))
        }
    }
    
     */
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
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}