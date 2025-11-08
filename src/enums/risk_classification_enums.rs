use crate::response::EnumResponse;
use strum_macros::EnumIter;

pub trait EnumMeta {
    fn code(&self) -> i32;
    fn display_name(&self) -> &'static str;
}
impl<T> From<T> for EnumResponse
where
    T: EnumMeta,
{
    fn from(value: T) -> Self {
        Self {
            code: value.code(),
            name: value.display_name().to_owned(),
        }
    }
}

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

#[repr(i32)]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum PotentialDamage {
    Negligible,
    Limited,
    Significant,
    LifeThreatening,
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
