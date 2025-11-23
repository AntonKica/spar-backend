use serde::Serialize;

pub mod risk_classification_enums;
pub mod risk_treatment_enums;
pub mod asset_enums;
pub mod fulfilled_threat_enums;
pub mod risk_analysis_process_enums;
pub mod step_2_threat_identification_enums;

#[macro_export]
macro_rules! int_enum {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[repr(i32)]
        #[derive(
            Debug,
            Copy,
            Clone,
            PartialEq,
            sqlx::Type,
            strum_macros::EnumIter,
            serde_repr::Deserialize_repr,
            serde_repr::Serialize_repr,
        )]
        pub enum $name {
            $($variant),*
        }
    };
}

pub trait EnumMeta {
    fn code(&self) -> i32;
    fn display_name(&self) -> &'static str;
}

#[derive(Serialize)]
pub struct EnumCodeName {
    pub code: i32,
    pub name: String,
}

impl<T> From<T> for EnumCodeName
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