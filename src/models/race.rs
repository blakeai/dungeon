use strum_macros::{AsRefStr, EnumIter, EnumString, EnumVariantNames};

#[derive(Debug, EnumIter, EnumString, EnumVariantNames, AsRefStr)]
pub enum Race {
    Elf,
    Dwarf,
    Human,
    HalfElf,
    Tiefling,
    Halfling,
    Gnome,
    Dragonborn,
    HalfOrc,
    Githyanki,
}

