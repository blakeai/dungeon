use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
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
