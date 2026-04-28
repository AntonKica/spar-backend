use actix_web::delete;
use crate::model::CreatedCode;
use crate::model::it_grundchutz_models::ItGrundschutzModule;
use crate::configuration::AppState;
use crate::service::ErrorResponse;
use crate::service::ApiError;
use actix_web::web::Path;
use utoipa_actix_web::service_config::ServiceConfig;
use crate::model::asset_model::{AssetModel, AssetDetailModel, AssetCreateModel};
use crate::service::GeneralService;
use crate::service::asset_service::AssetService;
use crate::service::ApiResult;
use actix_web::{get, post, web};
use crate::route::GeneralRoute;

pub struct AssetRoute;
use actix_web::web::Json;
use utoipa_actix_web::scope;

impl GeneralRoute for AssetRoute {
    fn configure(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/asset")
                .service(list_asset_modules)
                .service(list_asset)
                .service(create_asset)
                .service(detail_asset)
                .service(delete_asset)
        );
    }
}
#[utoipa::path(
    responses(
        (status = 200, description = "List all assets", body = Vec<AssetModel>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("")]
async fn list_asset(state: web::Data<AppState>) -> ApiResult<Json<Vec<AssetModel>>> {
    let assets = AssetService::list(&state.db).await?;
    Ok(Json(assets))
}
#[utoipa::path(
    responses(
        (status = 201, description = "Asset created", body = CreatedCode),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]#[post("")]
async fn create_asset(
    state: web::Data<AppState>,
    payload: Json<AssetCreateModel>,
) -> ApiResult<Json<CreatedCode>> {
    let mut tx = state.db.acquire().await?;
    let code = AssetService::create(&mut tx, payload.into_inner()).await?;
    Ok(Json(CreatedCode { code }))
}
#[utoipa::path(
    responses(
        (status = 200, description = "Asset detail", body = AssetDetailModel),
        (status = 404, description = "Asset not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{code}")]
async fn detail_asset(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<AssetDetailModel>> {
    let code = path.into_inner();
    let asset = AssetService::detail(&state.db, code.clone()).await?;
    asset
        .map(Json)
        .ok_or_else(|| ApiError::NotFound(format!("Asset with code {code} not found")))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Distinct modules from assets", body = Vec<ItGrundschutzModule>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/all/module")]
async fn list_asset_modules(
    state: web::Data<AppState>,
) -> ApiResult<Json<Vec<ItGrundschutzModule>>> {
    let modules = AssetService::distinct_modules(&state.db).await?;
    Ok(Json(modules))
}

#[utoipa::path(
    responses(
        (status = 204, description = "Asset deleted"),
        (status = 404, description = "Asset not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[delete("/{code}")]
async fn delete_asset(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<actix_web::HttpResponse> {
    let code = path.into_inner();
    let mut tx = state.db.acquire().await?;
    AssetService::delete(&mut tx, code).await?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}