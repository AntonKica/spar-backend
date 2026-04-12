use strum_macros::EnumIter;
use crate::enums::EnumMeta;

#[repr(i32)]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum FrequencyOfOccurrence {
    Rarely,
    Medium,
    Often,
    VeryOften,
}

impl EnumMeta for FrequencyOfOccurrence {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            FrequencyOfOccurrence::Rarely => "zriedkavo",
            FrequencyOfOccurrence::Medium => "mierne",
            FrequencyOfOccurrence::Often => "často",
            FrequencyOfOccurrence::VeryOften => "veľmi často",
        }
    }
}

impl From<i32> for FrequencyOfOccurrence {
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

#[repr(i32)]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum PotentialDamage {
    Negligible,
    Limited,
    Significant,
    LifeThreatening,
}

impl From<i32> for PotentialDamage {
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

impl EnumMeta for PotentialDamage {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            PotentialDamage::Negligible => "zanedbateľný",
            PotentialDamage::Limited => "obmedzený",
            PotentialDamage::Significant => "značný",
            PotentialDamage::LifeThreatening => "života ohrozujúci",
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum PotentialRisk {
    Low,
    Medium,
    High,
    VeryHigh,
}
impl EnumMeta for PotentialRisk {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            PotentialRisk::Low => "nízke",
            PotentialRisk::Medium => "stredné",
            PotentialRisk::High => "vysoké",
            PotentialRisk::VeryHigh => "veľmi vysoké",
        }
    }
}
