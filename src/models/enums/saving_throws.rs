use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum SavingThrow {
    #[serde(alias = "cha")]
    #[serde(alias = "charisma")]
    Charisma,
    #[serde(alias = "con")]
    #[serde(alias = "constitution")]
    Constitution,
    #[serde(alias = "dex")]
    #[serde(alias = "dexterity")]
    Dexterity,
    #[serde(alias = "int")]
    #[serde(alias = "intelligence")]
    Intelligence,
    #[serde(alias = "shields")]
    Shields,
    #[serde(alias = "str")]
    #[serde(alias = "strength")]
    Strength,
    #[serde(alias = "wis")]
    #[serde(alias = "wisdom")]
    Wisdom,
}
