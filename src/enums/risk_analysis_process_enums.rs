use strum_macros::EnumIter;
use crate::enums::EnumMeta;
use crate::enums::fulfilled_threat_enums::TimeCostUnit;

#[repr(i32)]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum ProcessStatus {
    Completed = 0,
    Failed,
    Waiting,
    InProgress,
}

impl EnumMeta for ProcessStatus {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            ProcessStatus::Completed => "dokončený",
            ProcessStatus::Waiting => "čaká",
            ProcessStatus::InProgress => "prebieha",
            ProcessStatus::Failed => "zrušený"
        }
    }
}