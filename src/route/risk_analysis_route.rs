use crate::service::risk_analysis_service::RiskClassificationUpdate;
use crate::service::risk_analysis_service::RiskMatrix;
use crate::service::risk_analysis_service::RiskClassificationDetail;
use crate::enums::RiskAnalysisState;
use crate::service::risk_analysis_service::ThreatWithModule;
use crate::model::it_grundchutz_models::ThreatModel;
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
                .service(get_risk_matrix)
                .service(list_risk_analyses)
                .service(create_risk_analysis)
                .service(list_risk_analysis_modules)
                .service(list_risk_analysis_modules_status)
                .service(list_risk_analysis_assets)
                .service(mark_module_threat_identification_done)
                .service(detail_risk_analysis)
                .service(sync_risk_analysis_threats)
                .service(list_risk_analysis_threats)
                .service(list_all_risk_analysis_threats)
                .service(complete_risk_analysis_step)
                .service(list_risk_classifications)
                .service(get_risk_classification)
                .service(update_risk_classification)
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

#[utoipa::path(
    responses(
        (status = 204, description = "Threats synced"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/sync-threats/{code}/{module}")]
async fn sync_risk_analysis_threats(
    state: web::Data<AppState>,
    path: Path<(String, String)>,
    payload: Json<Vec<String>>,
) -> ApiResult<actix_web::HttpResponse> {
    let (code, module) = path.into_inner();
    let mut tx = state.db.begin().await?;
    RiskAnalysisService::sync_threats(&mut tx, code, module, payload.into_inner()).await?;
    tx.commit().await?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}

#[utoipa::path(
    responses(
        (status = 200, description = "Linked threats for module in risk analysis", body = Vec<ThreatModel>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/list-threats/{code}/{module}")]
async fn list_risk_analysis_threats(
    state: web::Data<AppState>,
    path: Path<(String, String)>,
) -> ApiResult<Json<Vec<ThreatModel>>> {
    let (code, module) = path.into_inner();
    let threats = RiskAnalysisService::list_threats_by_module(&state.db, code, module).await?;
    Ok(Json(threats))
}
#[utoipa::path(
    responses(
        (status = 200, description = "All threats for risk analysis", body = Vec<ThreatWithModule>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/list-all-threats/{code}")]
async fn list_all_risk_analysis_threats(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<Vec<ThreatWithModule>>> {
    let code = path.into_inner();
    let threats = RiskAnalysisService::list_all_threats(&state.db, code).await?;
    Ok(Json(threats))
}
#[utoipa::path(
    responses(
        (status = 200, description = "Step completed", body = RiskAnalysisState),
        (status = 400, description = "Invalid state transition", body = ErrorResponse),
        (status = 404, description = "Risk analysis not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/complete-step/{code}/{state}")]
async fn complete_risk_analysis_step(
    state: web::Data<AppState>,
    path: Path<(String, String)>,
) -> ApiResult<Json<RiskAnalysisState>> {
    let (code, state_str) = path.into_inner();
    let expected_state: RiskAnalysisState = serde_json::from_value(
        serde_json::Value::String(state_str),
    )
        .map_err(|_| ApiError::Validation("Invalid state".to_string()))?;

    let mut tx = state.db.begin().await?;
    let next = RiskAnalysisService::complete_step(&mut tx, code, expected_state).await?;
    tx.commit().await?;
    Ok(Json(next))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Risk classifications", body = Vec<RiskClassificationDetail>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/list-risk-classifications/{code}")]
async fn list_risk_classifications(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<Vec<RiskClassificationDetail>>> {
    let code = path.into_inner();
    let rows = RiskAnalysisService::list_risk_classifications(&state.db, code).await?;
    Ok(Json(rows))
}
#[utoipa::path(
    responses(
        (status = 200, description = "Risk matrix", body = RiskMatrix)
    )
)]
#[get("/risk-matrix")]
async fn get_risk_matrix() -> Json<RiskMatrix> {
    Json(RiskAnalysisService::risk_matrix())
}

#[utoipa::path(
    responses(
        (status = 200, description = "Risk classification detail", body = RiskClassificationDetail),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/risk-classification/{code}/{module}/{threat}")]
async fn get_risk_classification(
    state: web::Data<AppState>,
    path: Path<(String, String, String)>,
) -> ApiResult<Json<RiskClassificationDetail>> {
    let (code, module, threat) = path.into_inner();
    let row = RiskAnalysisService::get_risk_classification(&state.db, code.clone(), module.clone(), threat.clone())
        .await?
        .ok_or_else(|| ApiError::NotFound(format!(
            "Risk classification not found for {code}/{module}/{threat}"
        )))?;
    Ok(Json(row))
}

#[utoipa::path(
    responses(
        (status = 204, description = "Risk classification updated"),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/risk-classification/{code}/{module}/{threat}")]
async fn update_risk_classification(
    state: web::Data<AppState>,
    path: Path<(String, String, String)>,
    payload: Json<RiskClassificationUpdate>,
) -> ApiResult<actix_web::HttpResponse> {
    let (code, module, threat) = path.into_inner();
    let mut tx = state.db.begin().await?;
    RiskAnalysisService::update_risk_classification(
        &mut tx, code, module, threat, payload.into_inner(),
    )
        .await?;
    tx.commit().await?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}