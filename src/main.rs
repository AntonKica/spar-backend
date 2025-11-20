use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::web::scope;
use actix_web::{App, HttpServer};
use spar_backend::configuration::{AppConfig, AppState};
use spar_backend::create_connection;
use spar_backend::route::{GeneralRoute};
use spar_backend::route::asset_route::AssetRoute;
use spar_backend::route::enum_route::EnumRoute;
use spar_backend::route::risk_analysis_process_route::RiskAnalysisProcessRoute;
use spar_backend::route::step_2_threat_identification_route::Step2ThreatIdenfiticationRoute;

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
                    .service(EnumRoute::routes())
                    .service(AssetRoute::routes())
                    .service(RiskAnalysisProcessRoute::routes())
                    .service(Step2ThreatIdenfiticationRoute::routes())
            )
    })
        .bind((config.server_host, config.server_port))?
        .run()
        .await
}