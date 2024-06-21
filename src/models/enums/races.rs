use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter, EnumString, EnumVariantNames};

#[derive(Serialize, Deserialize, Debug, EnumIter, EnumString, EnumVariantNames, AsRefStr)]
pub enum Race {
    #[serde(alias = "elf")]
    Elf,
    #[serde(alias = "dwarf")]
    Dwarf,
    #[serde(alias = "human")]
    Human,
    #[serde(alias = "half elf")]
    HalfElf,
    #[serde(alias = "tiefling")]
    Tiefling,
    #[serde(alias = "halfling")]
    Halfling,
    #[serde(alias = "gnome")]
    Gnome,
    #[serde(alias = "dragonborn")]
    Dragonborn,
    #[serde(alias = "half orc")]
    HalfOrc,
    #[serde(alias = "githyanki")]
    Githyanki,
}

