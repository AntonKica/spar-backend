use utoipa_actix_web::scope;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use spar_backend::configuration::{AppConfig, AppState};
use spar_backend::create_connection;
//use spar_backend::enums::risk_analysis_process_enums::ProcessStep::Step4RiskTreatment;
use spar_backend::route::{GeneralRoute};
use spar_backend::route::asset_route::AssetRoute;
use spar_backend::route::enum_route::EnumRoute;
use spar_backend::route::it_grundschutz_check_route::ItGrundschutCheckRoute;
use spar_backend::route::it_grundschutz_route::ItGrundschutzRoute;
use spar_backend::route::risk_analysis_route::RiskAnalysisRoute;
use spar_backend::route::security_measure_route::SecurityMeasureRoute;
use spar_backend::route::threat_route::ThreatRoute;
/*
use spar_backend::route::enum_route::EnumRoute;
use spar_backend::route::risk_analysis_process_route::RiskAnalysisProcessRoute;
use spar_backend::route::threat_route::SpecificThreatRoute;
use spar_backend::route::step_2_threat_identification_route::Step2ThreatIdenfiticationRoute;
use spar_backend::route::step_3_risk_classification_route::Step3RiskClassificationRoute;
use spar_backend::route::step_4_risk_treatment_route::Step4RiskTreatmentRoute;

 */

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let config = AppConfig::from_env();
    let db = create_connection(&config).await;

    HttpServer::new(move || {
        let (app, api) = App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .service(
                scope("/svc")
                    .configure(AssetRoute::configure)
                    .configure(ItGrundschutzRoute::configure)
                    .configure(ThreatRoute::configure)
                    .configure(EnumRoute::configure)
                    .configure(RiskAnalysisRoute::configure)
                    .configure(SecurityMeasureRoute::configure)
                    .configure(ItGrundschutCheckRoute::configure)
            )
            .split_for_parts();


        app.service(
            utoipa_swagger_ui::SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", api),
        )
    })
        .bind((config.server_host, config.server_port))?
        .run()
        .await
}