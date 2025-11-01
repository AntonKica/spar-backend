use actix_web::web::scope;
use actix_web::web;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use spar_backend::configuration::{AppConfig, AppState};
use spar_backend::create_connection;
use spar_backend::routes::{application_routes, business_process_list, business_process_routes, it_system_routes, other_routes, risk_analysis_process_routes, role_routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = AppConfig::from_env();
    let db = create_connection(&config).await;
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState{ db: db.clone() }))
            .service(
                scope("/svc")
                    .service(business_process_routes())
                    .service(role_routes())
                    .service(application_routes())
                    .service(it_system_routes())
                    .service(risk_analysis_process_routes())
                    .service(other_routes())
            )
    })
        .bind((config.server_host, config.server_port))?
        .run()
        .await
}