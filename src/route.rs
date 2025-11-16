pub mod enum_route;
pub mod asset_route;

// pub mod risk_analysis_process_route;
pub trait GeneralRoute {
    fn routes() -> actix_web::Scope;
}