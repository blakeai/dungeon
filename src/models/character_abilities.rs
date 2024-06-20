use super::{AbilityScore, AbilityType};
use std::collections::HashMap;
use derive_more::Constructor;

#[derive(Constructor, Debug)]
pub struct CharacterAbilities {
    pub abilities: HashMap<AbilityType, AbilityScore>,
}

impl CharacterAbilities {
    pub fn get(&self, ability: AbilityType) -> Option<&AbilityScore> {
        self.abilities.get(&ability)
    }

    pub fn get_cost(&self, ability: AbilityType) -> Option<i32> {
        self.abilities.get(&ability).map(|a| a.cost)
    }

    pub fn set(&mut self, ability: AbilityType, score: i32) {
        if let Some(a) = self.abilities.get_mut(&ability) {
            a.score = score;
        }
    }

    pub fn total_cost(&self) -> i32 {
        self.abilities.values().map(|a| a.cost).sum()
    }
}


#[cfg(test)]
pub(super) mod tests {
    use maplit::hashmap;
    use super::*;
    use crate::models::ability_type::AbilityType;

    pub(crate) fn create_test_character_abilities() -> CharacterAbilities {
        CharacterAbilities::new(hashmap! {
            AbilityType::Strength => AbilityScore::from_score(8),
            AbilityType::Dexterity => AbilityScore::from_score(14),
            AbilityType::Constitution => AbilityScore::from_score(12),
            AbilityType::Intelligence => AbilityScore::from_score(15),
            AbilityType::Wisdom => AbilityScore::from_score(13),
            AbilityType::Charisma => AbilityScore::from_score(10),
        })
    }

    #[test]
    fn test_create_test_character_abilities() {
        let abilities = create_test_character_abilities();
        assert_eq!(abilities.get(AbilityType::Strength).unwrap().score, 8);
        assert_eq!(abilities.get(AbilityType::Dexterity).unwrap().score, 14);
        assert_eq!(abilities.get(AbilityType::Constitution).unwrap().score, 12);
        assert_eq!(abilities.get(AbilityType::Intelligence).unwrap().score, 15);
        assert_eq!(abilities.get(AbilityType::Wisdom).unwrap().score, 13);
        assert_eq!(abilities.get(AbilityType::Charisma).unwrap().score, 10);
    }

    #[test]
    fn test_character_abilities_get_cost() {
        let abilities = create_test_character_abilities();
        assert_eq!(abilities.get_cost(AbilityType::Strength).unwrap(), 0);
        assert_eq!(abilities.get_cost(AbilityType::Dexterity).unwrap(), 7);
        assert_eq!(abilities.get_cost(AbilityType::Constitution).unwrap(), 4);
        assert_eq!(abilities.get_cost(AbilityType::Intelligence).unwrap(), 12);
        assert_eq!(abilities.get_cost(AbilityType::Wisdom).unwrap(), 5);
        assert_eq!(abilities.get_cost(AbilityType::Charisma).unwrap(), 2);
    }

    #[test]
    fn test_character_abilities_set() {
        let mut abilities = create_test_character_abilities();
        abilities.set(AbilityType::Strength, 10);
        assert_eq!(abilities.get(AbilityType::Strength).unwrap().score, 10);
    }

    #[test]
    fn test_character_abilities_total_cost() {
        let abilities = create_test_character_abilities();
        let actual = abilities.total_cost();
        assert_eq!(actual, 0 + // str
            7 + // dex
            4 + // con
            12 + // int
            5 + // wis
            2 // cha
        );
    }
}