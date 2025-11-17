use crate::enums::EnumMeta;
use crate::enums::fulfilled_threat_enums::TimeCostUnit;
use crate::int_enum;

use strum_macros::EnumIter;

int_enum! {
    ProcessStatus {
        InProgress,
        Finished,
        Terminated
    }
}

impl EnumMeta for ProcessStatus {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            ProcessStatus::InProgress => "prebieha",
            ProcessStatus::Finished => "ukončený",
            ProcessStatus::Terminated => "zrušený",
        }
    }
}

int_enum! {
ProcessStep {
    Step1SelectTour,
    Step2RelevantThreatIdentification,
    Step3RiskClassification,
    Step4RiskTreatment,
    Step5RiskTreatmentCheck,
    Step6Finished,
}
}

impl EnumMeta for ProcessStep {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            ProcessStep::Step1SelectTour => "1. Výber cieľových objektov na preskúmanie",
            ProcessStep::Step2RelevantThreatIdentification => "2. Identifikácia relevantných hrozieb",
            ProcessStep::Step3RiskClassification => "3. Klasifikácia rizík",
            ProcessStep::Step4RiskTreatment => "4. Ošetrenie rizík",
            ProcessStep::Step5RiskTreatmentCheck => "5. Kontrola rizík",
            ProcessStep::Step6Finished => "6. Ukončený"
        }
    }
}
