use crate::enums::EnumMeta;
use crate::int_enum;

int_enum! {
    RiskTreatmentType {
        Acceptance,
        Avoidance,
        Transfer,
        Reduction,
    }
}

impl EnumMeta for RiskTreatmentType {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            RiskTreatmentType::Acceptance => "akceptovanie",
            RiskTreatmentType::Avoidance => "vyhnutie sa",
            RiskTreatmentType::Transfer => "prenesenie",
            RiskTreatmentType::Reduction => "redukovanie/modifikácia",
        }
    }
}