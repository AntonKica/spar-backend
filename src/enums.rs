use serde::Serialize;
use strum_macros::EnumIter;
/*
pub mod risk_classification_enums;
pub mod risk_treatment_enums;
pub mod asset_enums;
pub mod fulfilled_threat_enums;
pub mod risk_analysis_process_enums;
pub mod step_2_threat_identification_enums;
pub mod step_3_risk_classification_enums;
pub mod step_4_risk_treatment_enums;

 */

#[macro_export]
macro_rules! int_enum {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[repr(i32)]
        #[derive(
            Debug,
            Copy,
            Clone,
            PartialEq,
            sqlx::Type,
            strum_macros::EnumIter,
            serde_repr::Deserialize_repr,
            serde_repr::Serialize_repr,
        )]
        pub enum $name {
            $($variant),*
        }
    };
}

pub trait EnumMeta {
    fn code(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct EnumCodeName {
    pub code: String,
    pub name: String,
}

impl<T> From<T> for EnumCodeName
where
    T: EnumMeta,
{
    fn from(value: T) -> Self {
        Self {
            code: value.code().to_owned(),
            name: value.display_name().to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, sqlx::Type, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[sqlx(type_name = "risk_analysis_state", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RiskAnalysisState {
    ThreatIdentification,
}

impl EnumMeta for RiskAnalysisState {
    fn code(&self) ->  &'static str {
        match self { RiskAnalysisState::ThreatIdentification => "threat_identification" }
    }

    fn display_name(&self) -> &'static str {
        match self {
            RiskAnalysisState::ThreatIdentification => "identifikácia hrozieb"
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, sqlx::Type, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[sqlx(type_name = "threat_category", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ThreatCategory {
    NaturalThreat,
    InfrastructureFailure,
    CompromiseOfFunctionsAndServices,
    HumanActions,
    PhysicalThreats,
    TechnicalFailures,
    OrganizationalThreats,
    Other,
}

impl EnumMeta for ThreatCategory {
    fn code(&self) -> &'static str {
        match self {
            ThreatCategory::NaturalThreat => "natural_threat",
            ThreatCategory::InfrastructureFailure => "infrastructure_failure",
            ThreatCategory::CompromiseOfFunctionsAndServices => "compromise_of_functions_and_services",
            ThreatCategory::HumanActions => "human_actions",
            ThreatCategory::PhysicalThreats => "physical_threats",
            ThreatCategory::TechnicalFailures => "technical_failures",
            ThreatCategory::OrganizationalThreats => "organizational_threats",
            ThreatCategory::Other => "other",
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            ThreatCategory::NaturalThreat => "Natural threats",
            ThreatCategory::InfrastructureFailure => "Infrastructure failures",
            ThreatCategory::CompromiseOfFunctionsAndServices => "Compromise of functions and services",
            ThreatCategory::HumanActions => "Human actions",
            ThreatCategory::PhysicalThreats => "Physical threats",
            ThreatCategory::TechnicalFailures => "Technical failures",
            ThreatCategory::OrganizationalThreats => "Organizational threats",
            ThreatCategory::Other => "Other",
        }
    }
}