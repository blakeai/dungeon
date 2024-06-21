use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum SavingThrow {
    #[serde(alias = "wis")]     Wisdom,
    #[serde(alias = "shields")] Shields,
    #[serde(alias = "cha")]     Charisma,
    #[serde(alias = "str")]     Strength,
    #[serde(alias = "dex")]     Dexterity,
    #[serde(alias = "con")]     Constitution,
    #[serde(alias = "int")]     Intelligence,
}
