use crate::enums::EnumMeta;
use crate::int_enum;

int_enum! {
    ThreatRelevance {
        Irrelevant,
        Indirect,
        Direct
    }
}

impl EnumMeta for ThreatRelevance {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            ThreatRelevance::Irrelevant => "irelevantná",
            ThreatRelevance::Indirect => "nepriamo",
            ThreatRelevance::Direct => "priamo",
        }
    }
}