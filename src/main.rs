use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::web::scope;
use actix_web::{App, HttpServer};
use spar_backend::configuration::{AppConfig, AppState};
use spar_backend::create_connection;
use spar_backend::route::application_route::ApplicationRoute;
use spar_backend::route::business_process_route::BusinessProcessRoute;
use spar_backend::route::{GeneralRoute, OtherRotes};
use spar_backend::route::it_system_route::ITSystemRoute;
use spar_backend::route::risk_analysis_process_route::RiskAnalysisProcessRoute;
use spar_backend::route::role::RoleRoute;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let config = AppConfig::from_env();
    let db = create_connection(&config).await;
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState{ db: db.clone() }))
            .service(
                scope("/svc")
                    .service(BusinessProcessRoute::routes())
                    .service(RoleRoute::routes())
                    .service(ApplicationRoute::routes())
                    .service(ITSystemRoute::routes())
                    .service(RiskAnalysisProcessRoute::routes())
                    .service(OtherRotes::routes())
            )
    })
        .bind((config.server_host, config.server_port))?
        .run()
        .await
}