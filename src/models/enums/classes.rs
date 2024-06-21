use derive_more::Display;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter, EnumString, EnumVariantNames};

#[derive(
    Serialize, Deserialize, Debug, Copy, Clone,
    EnumIter, EnumString, EnumVariantNames, AsRefStr,
    PartialEq, Eq, Hash, Display
)]
pub enum ClassType {
    #[serde(alias = "barbarian")]
    Barbarian,
    #[serde(alias = "bard")]
    Bard,
    #[serde(alias = "cleric")]
    Cleric,
    #[serde(alias = "druid")]
    Druid,
    #[serde(alias = "fighter")]
    Fighter,
    #[serde(alias = "monk")]
    Monk,
    #[serde(alias = "paladin")]
    Paladin,
    #[serde(alias = "ranger")]
    Ranger,
    #[serde(alias = "rogue")]
    Rogue,
    #[serde(alias = "sorcerer")]
    Sorcerer,
    #[serde(alias = "warlock")]
    Warlock,
    #[serde(alias = "wizard")]
    Wizard,
}
