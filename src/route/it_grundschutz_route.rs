use crate::model::it_grundchutz_models::ThreatModel;
use crate::configuration::AppState;
use actix_web::get;
use crate::service::it_grundschutz_service::ItGrundschutzService;
use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::web;
use crate::service::{ApiResult, ErrorResponse};
use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;
use crate::route::GeneralRoute;

pub struct ItGrundschutzRoute;

impl GeneralRoute for ItGrundschutzRoute {
    fn configure(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/it-grundschutz")
                .service(threats_by_module),
        );
    }
}
#[utoipa::path(
    responses(
        (status = 200, description = "Threats for the given module", body = Vec<ThreatModel>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/module/{code}/threats")]
async fn threats_by_module(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<Vec<ThreatModel>>> {
    let code = path.into_inner();
    let threats = ItGrundschutzService::threats_by_module(&state.db, code).await?;
    Ok(Json(threats))
}