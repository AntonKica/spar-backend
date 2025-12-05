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
            Self::Acceptance => "akceptovanie",
            Self::Avoidance => "vyhnutie sa",
            Self::Transfer => "prenesenie",
            Self::Reduction => "redukovanie/modifikácia",
        }
    }
}


impl From<i32> for RiskTreatmentType {
    fn from(val: i32) -> Self { 
        match val { 
            0 => Self::Acceptance,
            1 => Self::Avoidance,
            2 => Self::Transfer,
            3 => Self::Reduction,
            _ =>  Self::Acceptance
        }
    }
}

int_enum! {
    RiskTransferType {
        Outsourcing,
        Insurance,
    }
}

impl EnumMeta for RiskTransferType {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            Self::Outsourcing => "outsourcovanie",
            Self::Insurance => "poistenie",
        }
    }
}
