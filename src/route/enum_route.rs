use crate::enums::asset_enums::ProtectionNeeds;
use actix_web::ResponseError;
use strum::IntoEnumIterator;
use actix_web::{get, web, HttpResponse, Responder, Scope};
use actix_web::web::Path;
use serde::Serialize;
use crate::api::ApiResponse;
use crate::configuration::AppState;
use crate::enums::{ElementaryThreatRelevance, ModuleType};
use crate::enums::asset_enums::AssetType;
use crate::enums::risk_analysis_process_enums::{ProcessStatus, ProcessStep};
use crate::enums::risk_classification_enums::{FrequencyOfOccurrence, PotentialDamage, PotentialRisk};
use crate::enums::risk_treatment_enums::RiskTreatment;
use crate::response::EnumResponse;
use crate::route::GeneralRoute;
use crate::service::ApiError;

async fn enum_list<E: strum::IntoEnumIterator + Into<EnumResponse>>() -> impl Responder {
    let data: Vec<EnumResponse> = E::iter().map(Into::into).collect();
    HttpResponse::Ok().json(ApiResponse::new(data))
}

macro_rules! enum_handler {
    ($enum_type:ty) => {
        web::get().to(|| enum_list::<$enum_type>())
    };
}
pub struct EnumRoute {}

impl GeneralRoute for EnumRoute {
    fn routes() -> Scope {
        web::scope("/enum")
            .route("/protection-needs/", enum_handler!(ProtectionNeeds))
            .route("/frequency-of-occurrence/", enum_handler!(FrequencyOfOccurrence))
            .route("/potential-damage/", enum_handler!(PotentialDamage))
            .route("/potential-risk/", enum_handler!(PotentialRisk))
            .route("/elementary-threat-relevance/", enum_handler!(ElementaryThreatRelevance))
            .route("/risk-treatment/", enum_handler!(RiskTreatment))
            .route("/asset-type/", enum_handler!(AssetType))
            .route("/process-status/", enum_handler!(ProcessStatus))
            .route("/process-step/", enum_handler!(ProcessStep))
            .service(elementary_threat_list)
    }
}


#[derive(Serialize)]
struct EnumCodeResponse {
    code: String,
    name: String,
}
#[get("/elementary-threat/")]
async fn elementary_threat_list(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(EnumCodeResponse, r#"SELECT code, name FROM elementary_threat"#) .fetch_all(&data.db) .await;

    match query_result {
        Ok(res) => HttpResponse::Ok().json(ApiResponse::new(res)),
        Err(err) => ApiError::Database(err).error_response()
    }
}