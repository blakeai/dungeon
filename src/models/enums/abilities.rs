use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl FromStr for Ability {
    type Err = ();

    fn from_str(input: &str) -> Result<Ability, Self::Err> {
        match input {
            "str" | "strength" => Ok(Ability::Strength),
            "dex" | "dexterity" => Ok(Ability::Dexterity),
            "con" | "constitution" => Ok(Ability::Constitution),
            "int" | "intelligence" => Ok(Ability::Intelligence),
            "wis" | "wisdom" => Ok(Ability::Wisdom),
            "cha" | "charisma" => Ok(Ability::Charisma),
            _ => Err(()),
        }
    }
}