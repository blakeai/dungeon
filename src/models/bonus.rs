use super::ability_type::AbilityType;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Bonus {
    pub ability: AbilityType,
    pub value: i32,
}

impl Bonus {
    pub const fn new(ability: AbilityType, value: i32) -> Self {
        Self { ability, value }
    }
}