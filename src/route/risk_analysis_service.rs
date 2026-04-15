use crate::service::ApiError;
use crate::service::risk_analysis_service::RiskAnalysisModel;
use crate::service::ErrorResponse;
use crate::model::CreatedCode;
use crate::service::risk_analysis_service::RiskAnalysisService;
use crate::service::ApiResult;
use crate::configuration::AppState;
use actix_web::{get, post, web, web::Json, web::Path};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use crate::route::GeneralRoute;

pub struct RiskAnalysisRoute;

impl GeneralRoute for RiskAnalysisRoute {
    fn configure(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/risk-analysis")
                .service(list_risk_analyses)
                .service(create_risk_analysis)
                .service(detail_risk_analysis),
        );
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "List all risk analyses", body = Vec<RiskAnalysisModel>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("")]
async fn list_risk_analyses(
    state: web::Data<AppState>,
) -> ApiResult<Json<Vec<RiskAnalysisModel>>> {
    let analyses = RiskAnalysisService::list(&state.db).await?;
    Ok(Json(analyses))
}

#[utoipa::path(
    responses(
        (status = 201, description = "Risk analysis created", body = CreatedCode),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("")]
async fn create_risk_analysis(
    state: web::Data<AppState>,
) -> ApiResult<Json<CreatedCode>> {
    let mut tx = state.db.acquire().await?;
    let code = RiskAnalysisService::create(&mut tx).await?;
    Ok(Json(CreatedCode { code }))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Risk analysis detail", body = RiskAnalysisModel),
        (status = 404, description = "Risk analysis not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{code}")]
async fn detail_risk_analysis(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<RiskAnalysisModel>> {
    let code = path.into_inner();
    let analysis = RiskAnalysisService::detail(&state.db, code.clone()).await?;
    analysis
        .map(Json)
        .ok_or_else(|| ApiError::NotFound(format!("Risk analysis {code} not found")))
}