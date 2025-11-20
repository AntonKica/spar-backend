use crate::enums::EnumMeta;
use crate::enums::fulfilled_threat_enums::TimeCostUnit;
use crate::int_enum;

use strum_macros::EnumIter;

int_enum! {
    ElementaryThreatRelevance {
        Irrelevant,
        Indirect,
        Direct
    }
}

impl EnumMeta for ElementaryThreatRelevance {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            ElementaryThreatRelevance::Irrelevant => "irelevantná",
            ElementaryThreatRelevance::Indirect => "nepriamo",
            ElementaryThreatRelevance::Direct => "priamo",
        }
    }
}