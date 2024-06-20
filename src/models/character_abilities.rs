use std::collections::HashMap;
use derive_more::Constructor;
use crate::models::ability_score::AbilityScore;
use crate::models::enums::abilities::Ability;

#[derive(Constructor, Debug)]
pub struct CharacterAbilities {
    pub abilities: HashMap<Ability, AbilityScore>,
}

impl CharacterAbilities {
    pub fn get(&self, ability: Ability) -> Option<&AbilityScore> {
        self.abilities.get(&ability)
    }

    pub fn get_cost(&self, ability: Ability) -> Option<i32> {
        self.abilities.get(&ability).map(|a| a.cost)
    }

    pub fn set(&mut self, ability: Ability, score: i32) {
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
    use crate::models::enums::abilities::Ability;

    pub(crate) fn create_test_character_abilities() -> CharacterAbilities {
        CharacterAbilities::new(hashmap! {
            Ability::Strength => AbilityScore::from_score(8),
            Ability::Dexterity => AbilityScore::from_score(14),
            Ability::Constitution => AbilityScore::from_score(12),
            Ability::Intelligence => AbilityScore::from_score(15),
            Ability::Wisdom => AbilityScore::from_score(13),
            Ability::Charisma => AbilityScore::from_score(10),
        })
    }

    #[test]
    fn test_create_test_character_abilities() {
        let abilities = create_test_character_abilities();
        assert_eq!(abilities.get(Ability::Strength).unwrap().score, 8);
        assert_eq!(abilities.get(Ability::Dexterity).unwrap().score, 14);
        assert_eq!(abilities.get(Ability::Constitution).unwrap().score, 12);
        assert_eq!(abilities.get(Ability::Intelligence).unwrap().score, 15);
        assert_eq!(abilities.get(Ability::Wisdom).unwrap().score, 13);
        assert_eq!(abilities.get(Ability::Charisma).unwrap().score, 10);
    }

    #[test]
    fn test_character_abilities_get_cost() {
        let abilities = create_test_character_abilities();
        assert_eq!(abilities.get_cost(Ability::Strength).unwrap(), 0);
        assert_eq!(abilities.get_cost(Ability::Dexterity).unwrap(), 7);
        assert_eq!(abilities.get_cost(Ability::Constitution).unwrap(), 4);
        assert_eq!(abilities.get_cost(Ability::Intelligence).unwrap(), 12);
        assert_eq!(abilities.get_cost(Ability::Wisdom).unwrap(), 5);
        assert_eq!(abilities.get_cost(Ability::Charisma).unwrap(), 2);
    }

    #[test]
    fn test_character_abilities_set() {
        let mut abilities = create_test_character_abilities();
        abilities.set(Ability::Strength, 10);
        assert_eq!(abilities.get(Ability::Strength).unwrap().score, 10);
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