use strum_macros::EnumIter;
use crate::enums::EnumMeta;

#[repr(i32)]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum TimeCostUnit {
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

impl EnumMeta for TimeCostUnit {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            TimeCostUnit::Hours => "hodiny",
            TimeCostUnit::Days => "dni",
            TimeCostUnit::Weeks => "týždne",
            TimeCostUnit::Months => "mesiace",
            TimeCostUnit::Years => "roky",
        }
    }
}
