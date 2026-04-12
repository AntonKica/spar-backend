use crate::enums::EnumMeta;
use crate::int_enum;

int_enum! {
    ThreatProbability {
        Rarely,
        Medium,
        Often,
        VeryOften,
    }
}

impl EnumMeta for ThreatProbability {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            ThreatProbability::Rarely => "zriedkavo",
            ThreatProbability::Medium => "stredne",
            ThreatProbability::Often => "často",
            ThreatProbability::VeryOften => "veľmi často",
        }
    }
}

impl From<i32> for ThreatProbability {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Rarely,
            1 => Self::Medium,
            2 => Self::Often,
            3 => Self::VeryOften,
            _ => Self::Rarely
        }
    }
}

int_enum! {
    ThreatImpact {
        Negligible,
        Limited,
        Significant,
        LifeThreatening,
    }
}

impl From<i32> for ThreatImpact {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Negligible,
            1 => Self::Limited,
            2 => Self::Significant,
            3 => Self::LifeThreatening,
            _ => Self::Negligible
        }
    }
}

impl EnumMeta for ThreatImpact {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            ThreatImpact::Negligible => "zanedbateľný",
            ThreatImpact::Limited => "obmedzený",
            ThreatImpact::Significant => "významný",
            ThreatImpact::LifeThreatening => "život ohrozujúci",
        }
    }
}

int_enum! {
    ThreatRisk {
        Low,
        Medium,
        High,
        VeryHigh,
    }
}

impl EnumMeta for ThreatRisk {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            ThreatRisk::Low => "nízke",
            ThreatRisk::Medium => "stredné",
            ThreatRisk::High => "vysoké",
            ThreatRisk::VeryHigh => "veľmi vysoké",
        }
    }
}
