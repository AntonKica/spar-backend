use crate::api::ApiResponse;
use crate::enums::asset_enums::AssetType;
use crate::enums::asset_enums::ProtectionNeeds;
use crate::enums::risk_analysis_process_enums::{ProcessStatus, ProcessStep};
use crate::enums::risk_classification_enums::{FrequencyOfOccurrence, PotentialDamage, PotentialRisk};
use crate::enums::risk_treatment_enums::RiskTreatment;
use crate::enums::EnumCodeName;
use crate::route::GeneralRoute;
use actix_web::{web, HttpResponse, Responder, Scope};
use crate::enums::step_2_threat_identification_enums::ThreatRelevance;

async fn enum_list<E: strum::IntoEnumIterator + Into<EnumCodeName>>() -> impl Responder {
    let data: Vec<EnumCodeName> = E::iter().map(Into::into).collect();
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
            .route("/threat-relevance/", enum_handler!(ThreatRelevance))
            .route("/risk-treatment/", enum_handler!(RiskTreatment))
            .route("/asset-type/", enum_handler!(AssetType))
            .route("/process-status/", enum_handler!(ProcessStatus))
            .route("/process-step/", enum_handler!(ProcessStep))
    }
}