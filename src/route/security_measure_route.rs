use crate::enums::RiskTreatmentType;
use crate::service::ApiError;
use actix_web::web::Path;
use actix_web::web;
use crate::service::security_measure_service::SecurityMeasure;
use crate::service::security_measure_service::SecurityMeasureService;
use crate::service::security_measure_service::SecurityMeasureCreate;
use actix_web::get;
use crate::configuration::AppState;
use utoipa_actix_web::scope;
use crate::service::ApiResult;
use actix_web::web::Json;
use actix_web::post;
use crate::service::ErrorResponse;
use crate::model::CreatedCode;
use utoipa_actix_web::service_config::ServiceConfig;
use crate::route::GeneralRoute;

pub struct SecurityMeasureRoute;

impl GeneralRoute for SecurityMeasureRoute {
    fn configure(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/security-measure")
                .service(create_security_measure)
                .service(list_security_measures)
                .service(list_security_measures_by_treatment),
        );
    }
}

#[utoipa::path(
    responses(
        (status = 201, description = "Security measure created", body = CreatedCode),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("")]
async fn create_security_measure(
    state: web::Data<AppState>,
    payload: Json<SecurityMeasureCreate>,
) -> ApiResult<Json<CreatedCode>> {
    let mut tx = state.db.acquire().await?;
    let code = SecurityMeasureService::create(&mut tx, payload.into_inner()).await?;
    Ok(Json(CreatedCode { code }))
}

#[utoipa::path(
    responses(
        (status = 200, description = "All security measures", body = Vec<SecurityMeasure>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("")]
async fn list_security_measures(
    state: web::Data<AppState>,
) -> ApiResult<Json<Vec<SecurityMeasure>>> {
    let rows = SecurityMeasureService::list(&state.db).await?;
    Ok(Json(rows))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Security measures by treatment type", body = Vec<SecurityMeasure>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/by-treatment/{treatment}")]
async fn list_security_measures_by_treatment(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<Vec<SecurityMeasure>>> {
    let treatment: RiskTreatmentType = serde_json::from_value(
        serde_json::Value::String(path.into_inner()),
    )
        .map_err(|_| ApiError::Validation("Invalid treatment type".to_string()))?;

    let rows = SecurityMeasureService::list_by_treatment(&state.db, treatment).await?;
    Ok(Json(rows))
}