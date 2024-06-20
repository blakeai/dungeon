use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum SavingThrow {
    Charisma,
    Constitution,
    Dexterity,
    Intelligence,
    Shields,
    Strength,
    Wisdom,
}
