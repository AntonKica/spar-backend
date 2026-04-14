use crate::service::ApiError;
use crate::model::it_grundchutz_models::ThreatModel;
use crate::service::ErrorResponse;
use crate::service::GeneralService;
use crate::configuration::AppState;
use crate::service::threat_service::ThreatService;
use crate::service::ApiResult;
use crate::service::threat_service::ThreatModelCreate;
use crate::model::CreatedCode;
use actix_web::{get, post, delete, web, web::Json, web::Path};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use crate::route::GeneralRoute;

pub struct ThreatRoute;

impl GeneralRoute for ThreatRoute {
    fn configure(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/threat")
                .service(list_threat)
                .service(create_threat)
                .service(detail_threat)
                .service(delete_threat),
        );
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "List all threats", body = Vec<ThreatModel>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("")]
async fn list_threat(state: web::Data<AppState>) -> ApiResult<Json<Vec<ThreatModel>>> {
    let threats = ThreatService::list(&state.db).await?;
    Ok(Json(threats))
}

#[utoipa::path(
    responses(
        (status = 201, description = "Threat created", body = CreatedCode),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("")]
async fn create_threat(
    state: web::Data<AppState>,
    payload: Json<ThreatModelCreate>,
) -> ApiResult<Json<CreatedCode>> {
    let mut tx = state.db.acquire().await?;
    let code = ThreatService::create(&mut tx, payload.into_inner()).await?;
    Ok(Json(CreatedCode { code }))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Threat detail", body = ThreatModel),
        (status = 404, description = "Threat not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{code}")]
async fn detail_threat(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<ThreatModel>> {
    let code = path.into_inner();
    let threat = ThreatService::detail(&state.db, code.clone()).await?;
    threat
        .map(Json)
        .ok_or_else(|| ApiError::NotFound(format!("Threat with code {code} not found")))
}

#[utoipa::path(
    responses(
        (status = 204, description = "Threat deleted"),
        (status = 400, description = "Cannot delete elementary threat", body = ErrorResponse),
        (status = 404, description = "Threat not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[delete("/{code}")]
async fn delete_threat(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<actix_web::HttpResponse> {
    let code = path.into_inner();
    let mut tx = state.db.acquire().await?;
    ThreatService::delete(&mut tx, code).await?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}