use strum_macros::EnumIter;
use crate::enums::EnumMeta;

#[repr(i32)]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum RiskTreatment {
    Avoidance,
    Reduction,
    Transfer,
    Acceptance,
}

impl EnumMeta for RiskTreatment {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            RiskTreatment::Avoidance => "vyhnutie sa riziku",
            RiskTreatment::Reduction => "prijatie dodatočných opatrení",
            RiskTreatment::Transfer => "prenesenie rizika",
            RiskTreatment::Acceptance => "akceptovanie rizika"
        }
    }
}
