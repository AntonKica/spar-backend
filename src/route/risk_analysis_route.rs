use crate::service::risk_analysis_service::ModuleWithStatus;
use crate::model::it_grundchutz_models::ItGrundschutzModule;
use crate::model::asset_model::AssetModel;
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
                .service(list_risk_analysis_modules)
                .service(list_risk_analysis_modules_status)
                .service(list_risk_analysis_assets)
                .service(mark_module_threat_identification_done)
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

#[utoipa::path(
    responses(
        (status = 200, description = "Modules for risk analysis", body = Vec<ItGrundschutzModule>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/list-modules/{code}")]
async fn list_risk_analysis_modules(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<Vec<ItGrundschutzModule>>> {
    let code = path.into_inner();
    let modules = RiskAnalysisService::list_modules_by_code(&state.db, code).await?;
    Ok(Json(modules))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Assets for risk analysis", body = Vec<AssetModel>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/list-assets/{code}")]
async fn list_risk_analysis_assets(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<Vec<AssetModel>>> {
    let code = path.into_inner();
    let assets = RiskAnalysisService::list_assets_by_code(&state.db, code).await?;
    Ok(Json(assets))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Modules with threat identification status", body = Vec<ModuleWithStatus>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/list-modules-status/{code}")]
async fn list_risk_analysis_modules_status(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<Vec<ModuleWithStatus>>> {
    let code = path.into_inner();
    let modules = RiskAnalysisService::list_modules_with_status(&state.db, code).await?;
    Ok(Json(modules))
}

#[utoipa::path(
    responses(
        (status = 204, description = "Module threat identification marked as done"),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/mark-done/{code}/{module}")]
async fn mark_module_threat_identification_done(
    state: web::Data<AppState>,
    path: Path<(String, String)>,
) -> ApiResult<actix_web::HttpResponse> {
    let (code, module) = path.into_inner();
    let mut tx = state.db.acquire().await?;
    RiskAnalysisService::set_module_threat_identification_done(&mut tx, code, module).await?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}