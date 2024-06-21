use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum Ability {
    #[serde(rename = "wis")] Wisdom,
    #[serde(rename = "cha")] Charisma,
    #[serde(rename = "str")] Strength,
    #[serde(rename = "dex")] Dexterity,
    #[serde(rename = "int")] Intelligence,
    #[serde(rename = "con")] Constitution,
}
