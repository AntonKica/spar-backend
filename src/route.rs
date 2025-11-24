pub mod enum_route;
pub mod asset_route;
pub mod threat_route;
pub mod risk_analysis_process_route;
pub mod step_2_threat_identification_route;
pub mod step_3_risk_classification_route;

pub trait GeneralRoute {
    fn routes() -> actix_web::Scope;
}