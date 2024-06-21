use heck::ToShoutySnakeCase;
use strum_macros::{EnumString, EnumVariantNames};

// lowercase
#[derive(EnumVariantNames, EnumString, Debug)]
pub enum Environment {
    Test,
    Live,
}

// SHOUTY_UPPER_SNAKE_CASE
#[derive(EnumVariantNames, EnumString, Debug)]
pub enum EnvVar {
    PointBuyConfigFilename,
    CharacterClassConfigFilename,
    Port,
}

pub trait ToKey {
    fn to_key(&self) -> String;
}

impl ToKey for Environment {
    fn to_key(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

impl ToKey for EnvVar {
    fn to_key(&self) -> String {
        format!("{:?}", self).to_shouty_snake_case()
    }
}