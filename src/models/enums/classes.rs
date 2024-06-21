use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
pub enum ClassType {
    #[serde(alias = "bard")]       Bard,
    #[serde(alias = "monk")]       Monk,
    #[serde(alias = "druid")]      Druid,
    #[serde(alias = "rogue")]      Rogue,
    #[serde(alias = "cleric")]     Cleric,
    #[serde(alias = "ranger")]     Ranger,
    #[serde(alias = "wizard")]     Wizard,
    #[serde(alias = "paladin")]    Paladin,
    #[serde(alias = "fighter")]    Fighter,
    #[serde(alias = "warlock")]    Warlock,
    #[serde(alias = "sorcerer")]   Sorcerer,
    #[serde(alias = "barbarian")]  Barbarian,
}
