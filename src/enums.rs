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
 */

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

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, sqlx::Type, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[sqlx(type_name = "protection_requirement", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProtectionRequirement {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl EnumMeta for ProtectionRequirement {
    fn code(&self) -> &'static str {
        match self {
            ProtectionRequirement::Low => "low",
            ProtectionRequirement::Medium => "medium",
            ProtectionRequirement::High => "high",
            ProtectionRequirement::VeryHigh => "very_high",
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            ProtectionRequirement::Low => "nízka",
            ProtectionRequirement::Medium => "stredná",
            ProtectionRequirement::High => "vysoká",
            ProtectionRequirement::VeryHigh => "veľmi vysoká",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, sqlx::Type, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[sqlx(type_name = "risk_analysis_state", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RiskAnalysisState {
    ThreatIdentification,
    RiskClassification,
    RiskTreatment,
    ItGrundschutzCheck,
    Done,
}

impl RiskAnalysisState {
    pub fn next(self) -> Option<Self> {
        match self {
            Self::ThreatIdentification => Some(Self::RiskClassification),
            Self::RiskClassification => Some(Self::RiskTreatment),
            Self::RiskTreatment => Some(Self::ItGrundschutzCheck),
            Self::ItGrundschutzCheck => Some(Self::Done),
            Self::Done => None
        }
    }
}

impl EnumMeta for RiskAnalysisState {
    fn code(&self) ->  &'static str {
        match self {
            RiskAnalysisState::ThreatIdentification => "threat_identification",
            RiskAnalysisState::RiskClassification => "risk_classification",
            RiskAnalysisState::RiskTreatment => "risk_treatment",
            RiskAnalysisState::ItGrundschutzCheck => "it_grundschutz_check",
            RiskAnalysisState::Done => "done"
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            RiskAnalysisState::ThreatIdentification => "identifikácia hrozieb",
            RiskAnalysisState::RiskClassification => "klasifikácia rizík",
            RiskAnalysisState::RiskTreatment => "ošetrenie rizík",
            RiskAnalysisState::ItGrundschutzCheck => "it grundschutz check",
            RiskAnalysisState::Done => "ukončené"
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

#[derive(Debug, Clone, Copy, EnumIter, sqlx::Type, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[sqlx(type_name = "likelihood", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Likelihood {
    Rarely,
    Medium,
    Often,
    VeryOften,
}

#[derive(Debug, Clone, Copy, EnumIter, sqlx::Type, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[sqlx(type_name = "impact", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Impact {
    Negligible,
    Limited,
    Significant,
    LifeThreatening,
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, sqlx::Type, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[sqlx(type_name = "risk", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Risk {
    Low,
    Medium,
    High,
    VeryHigh,
}
#[derive(Debug, Clone, Copy, EnumIter, PartialEq, sqlx::Type, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[sqlx(type_name = "risk_treatment_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RiskTreatmentType {
    Avoid,
    Reduce,
    Transfer,
    Accept,
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, sqlx::Type, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[sqlx(type_name = "implementation_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ImplementationStatus {
    NotAssessed,
    None,
    Partial,
    Full,
    Redundant,
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
            ThreatCategory::NaturalThreat => "Prírodné hrozby",
            ThreatCategory::InfrastructureFailure => "Zlyhania infraštruktúry",
            ThreatCategory::CompromiseOfFunctionsAndServices => "Kompromitácia funkcií a služieb",
            ThreatCategory::HumanActions => "Ľudské činy",
            ThreatCategory::PhysicalThreats => "Fyzické hrozby",
            ThreatCategory::TechnicalFailures => "Technické zlyhania",
            ThreatCategory::OrganizationalThreats => "Organizačné hrozby",
            ThreatCategory::Other => "Ostatné",
        }
    }
}
impl EnumMeta for Likelihood {
    fn code(&self) -> &'static str {
        match self {
            Likelihood::Rarely => "rarely",
            Likelihood::Medium => "medium",
            Likelihood::Often => "often",
            Likelihood::VeryOften => "very_often",
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            Likelihood::Rarely => "zriedkavo",
            Likelihood::Medium => "stredne",
            Likelihood::Often => "často",
            Likelihood::VeryOften => "veľmi často",
        }
    }
}

impl EnumMeta for Impact {
    fn code(&self) -> &'static str {
        match self {
            Impact::Negligible => "negligible",
            Impact::Limited => "limited",
            Impact::Significant => "significant",
            Impact::LifeThreatening => "life_threatening",
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            Impact::Negligible => "zanedbateľný",
            Impact::Limited => "obmedzený",
            Impact::Significant => "značný",
            Impact::LifeThreatening => "život ohrozujúci",
        }
    }
}

impl EnumMeta for Risk {
    fn code(&self) -> &'static str {
        match self {
            Risk::Low => "low",
            Risk::Medium => "medium",
            Risk::High => "high",
            Risk::VeryHigh => "very_high",
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            Risk::Low => "nízke",
            Risk::Medium => "stredné",
            Risk::High => "vysoké",
            Risk::VeryHigh => "veľmi vysoké",
        }
    }
}

impl Risk {
    pub fn from_matrix(likelihood: Likelihood, impact: Impact) -> Self {
        match (likelihood, impact) {
            (Likelihood::Rarely,    Impact::Negligible)      => Risk::Low,
            (Likelihood::Rarely,    Impact::Limited)         => Risk::Low,
            (Likelihood::Rarely,    Impact::Significant)     => Risk::Medium,
            (Likelihood::Rarely,    Impact::LifeThreatening) => Risk::Medium,
            (Likelihood::Medium,    Impact::Negligible)      => Risk::Low,
            (Likelihood::Medium,    Impact::Limited)         => Risk::Low,
            (Likelihood::Medium,    Impact::Significant)     => Risk::Medium,
            (Likelihood::Medium,    Impact::LifeThreatening) => Risk::High,
            (Likelihood::Often,     Impact::Negligible)      => Risk::Low,
            (Likelihood::Often,     Impact::Limited)         => Risk::Medium,
            (Likelihood::Often,     Impact::Significant)     => Risk::High,
            (Likelihood::Often,     Impact::LifeThreatening) => Risk::VeryHigh,
            (Likelihood::VeryOften, Impact::Negligible)      => Risk::Low,
            (Likelihood::VeryOften, Impact::Limited)         => Risk::High,
            (Likelihood::VeryOften, Impact::Significant)     => Risk::VeryHigh,
            (Likelihood::VeryOften, Impact::LifeThreatening) => Risk::VeryHigh,
        }
    }
}

impl EnumMeta for RiskTreatmentType {
    fn code(&self) -> &'static str {
        match self {
            RiskTreatmentType::Avoid => "avoid",
            RiskTreatmentType::Reduce => "reduce",
            RiskTreatmentType::Transfer => "transfer",
            RiskTreatmentType::Accept => "accept",
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            RiskTreatmentType::Avoid => "vyhnúť sa",
            RiskTreatmentType::Reduce => "redukovať",
            RiskTreatmentType::Transfer => "preniesť",
            RiskTreatmentType::Accept => "akceptovať",
        }
    }
}

impl EnumMeta for ImplementationStatus {
    fn code(&self) -> &'static str {
        match self {
            ImplementationStatus::NotAssessed => "not_assessed",
            ImplementationStatus::None => "none",
            ImplementationStatus::Partial => "partial",
            ImplementationStatus::Full => "full",
            ImplementationStatus::Redundant => "redundant",
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            ImplementationStatus::NotAssessed => "nehodnotené",
            ImplementationStatus::None => "neimplementované",
            ImplementationStatus::Partial => "čiastočne",
            ImplementationStatus::Full => "úplne",
            ImplementationStatus::Redundant => "nadbytočné",
        }
    }
}
