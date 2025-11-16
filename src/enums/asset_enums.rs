use strum_macros::EnumIter;
use crate::enums::EnumMeta;

#[repr(i32)]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum AssetType {
    BusinessProcess,
    ItSystem,
    ItApplication,
    CommunicationsChannel,
    Room,
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
            AssetType::Room => "miestnosť"
        }
    }
}
