use crate::enums::EnumMeta;
use crate::int_enum;
use strum_macros::EnumIter;

int_enum! {
    AssetType {
        BusinessProcess,
        ItSystem,
        ItApplication,
        CommunicationsChannel,
        Room,
    }
}

impl EnumMeta for AssetType {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            AssetType::BusinessProcess => "biznisový proces",
            AssetType::ItSystem => "it systém",
            AssetType::ItApplication => "it aplikácia",
            AssetType::CommunicationsChannel => "komunikačný kanál",
            AssetType::Room => "miestnosť",
        }
    }
}

int_enum! {
    ProtectionNeeds {
        Normal,
        High,
        VeryHigh,
    }
}

impl EnumMeta for ProtectionNeeds {
    fn code(&self) -> i32 {
        *self as i32
    }

    fn display_name(&self) -> &'static str {
        match self {
            ProtectionNeeds::Normal => "normálna",
            ProtectionNeeds::High => "vysoká",
            ProtectionNeeds::VeryHigh => "veľmi vysoká",
        }
    }
}
