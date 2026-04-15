use utoipa_actix_web::service_config::ServiceConfig;

pub mod asset_route;
pub mod it_grundschutz_route;
pub mod threat_route;
pub mod risk_analysis_service;
pub mod enum_route;
/*
pub mod enum_route;
pub mod threat_route;
pub mod risk_analysis_process_route;
pub mod step_2_threat_identification_route;
pub mod step_3_risk_classification_route;
pub mod step_4_risk_treatment_route;
 */

pub trait GeneralRoute {
    fn configure(cfg: &mut ServiceConfig);
}