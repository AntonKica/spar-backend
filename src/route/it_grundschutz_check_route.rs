use actix_web::web;
use crate::service::ErrorResponse;
use crate::service::it_grundschutz_check_service::{FullAssessment, ItGrundschutCheckService};
use actix_web::{get, post };
use actix_web::web::{Json, Path};
use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;
use crate::configuration::AppState;
use crate::route::GeneralRoute;
use crate::service::ApiResult;
use crate::service::risk_analysis_service::{RiskAssessmentUpdateModel};

pub struct ItGrundschutCheckRoute;
impl GeneralRoute for ItGrundschutCheckRoute {
    fn configure(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/it-grundschutz-check")
                .service(get_full_assessment)
                .service(update_security_measure_assessment)
                .service(update_requirement_assessment)
                .service(is_assessment_completed)
        );
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Full assessment", body = FullAssessment),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/full-assessment/{code}")]
async fn get_full_assessment(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<FullAssessment>> {
    let code = path.into_inner();
    let assessment = ItGrundschutCheckService::full_assessment(&state.db, code).await?;
    Ok(Json(assessment))
}

#[utoipa::path(
    responses(
        (status = 204, description = "Security measure assessment updated"),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/assessment/security-measure/{id}")]
async fn update_security_measure_assessment(
    state: web::Data<AppState>,
    path: Path<uuid::Uuid>,
    payload: Json<RiskAssessmentUpdateModel>,
) -> ApiResult<actix_web::HttpResponse> {
    let id = path.into_inner();
    let mut tx = state.db.begin().await?;
    ItGrundschutCheckService::update_security_measure_assessment(
        &mut tx, id, payload.into_inner(),
    )
        .await?;
    tx.commit().await?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}

#[utoipa::path(
    responses(
        (status = 204, description = "Requirement assessment updated"),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/assessment/requirement/{id}")]
async fn update_requirement_assessment(
    state: web::Data<AppState>,
    path: Path<uuid::Uuid>,
    payload: Json<RiskAssessmentUpdateModel>,
) -> ApiResult<actix_web::HttpResponse> {
    let id = path.into_inner();
    let mut tx = state.db.begin().await?;
    ItGrundschutCheckService::update_requirement_assessment(
        &mut tx, id, payload.into_inner(),
    )
        .await?;
    tx.commit().await?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}

#[utoipa::path(
    responses(
        (status = 200, description = "Assessment completion status", body = bool),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/assessment-completed/{code}")]
async fn is_assessment_completed(
    state: web::Data<AppState>,
    path: Path<String>,
) -> ApiResult<Json<bool>> {
    let code = path.into_inner();
    let completed = ItGrundschutCheckService::is_risk_treatment_assessment_completed(&state.db, code).await?;
    Ok(Json(completed))
}
