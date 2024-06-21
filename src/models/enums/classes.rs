use strum_macros::{AsRefStr, EnumIter, EnumString, EnumVariantNames};

#[derive(Debug, Copy, Clone, EnumIter, EnumString, EnumVariantNames, AsRefStr, PartialEq, Eq, Hash)]
pub enum ClassType {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

