use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum Ability {
    #[serde(rename = "str")]
    #[serde(alias = "strength")]
    Strength,
    #[serde(rename = "dex")]
    #[serde(alias = "dexterity")]
    Dexterity,
    #[serde(rename = "con")]
    #[serde(alias = "constitution")]
    Constitution,
    #[serde(rename = "int")]
    #[serde(alias = "intelligence")]
    Intelligence,
    #[serde(rename = "wis")]
    #[serde(alias = "wisdom")]
    Wisdom,
    #[serde(rename = "cha")]
    #[serde(alias = "charisma")]
    Charisma,
}
