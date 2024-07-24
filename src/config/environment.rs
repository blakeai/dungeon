use heck::ToShoutySnakeCase;
use strum_macros::{EnumString, VariantNames};

// SHOUTY_UPPER_SNAKE_CASE
#[derive(VariantNames, EnumString, Debug)]
pub enum EnvVar {
    LogLevel,
    PointBuyConfigFilename,
    CharacterClassConfigFilename,
    Port,
}

pub trait ToKey {
    fn to_key(&self) -> String;
}

impl ToKey for EnvVar {
    fn to_key(&self) -> String {
        format!("{:?}", self).to_shouty_snake_case()
    }
}