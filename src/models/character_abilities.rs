use std::collections::HashMap;

use crate::models::ability_score::AbilityScore;
use crate::models::enums::abilities::Ability;

#[derive(Debug)]
pub struct CharacterAbilities {
    pub abilities: HashMap<Ability, AbilityScore>,
}

#[allow(dead_code)]
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

    use crate::models::enums::abilities::Ability;

    use super::*;

    pub(crate) fn create_test_character_abilities() -> CharacterAbilities {
        CharacterAbilities {
            abilities: hashmap! {
                Ability::Strength => AbilityScore::from_score(8),
                Ability::Dexterity => AbilityScore::from_score(14),
                Ability::Constitution => AbilityScore::from_score(12),
                Ability::Intelligence => AbilityScore::from_score(15),
                Ability::Wisdom => AbilityScore::from_score(13),
                Ability::Charisma => AbilityScore::from_score(10),
            }
        }
    }

    #[test]
    fn test_create_test_character_abilities() {
        let abilities = create_test_character_abilities();
        assert_eq!(8, abilities.get(Ability::Strength).unwrap().score);
        assert_eq!(14, abilities.get(Ability::Dexterity).unwrap().score);
        assert_eq!(12, abilities.get(Ability::Constitution).unwrap().score);
        assert_eq!(15, abilities.get(Ability::Intelligence).unwrap().score);
        assert_eq!(13, abilities.get(Ability::Wisdom).unwrap().score);
        assert_eq!(10, abilities.get(Ability::Charisma).unwrap().score);
    }

    #[test]
    fn test_character_abilities_get_cost() {
        let abilities = create_test_character_abilities();
        assert_eq!(0, abilities.get_cost(Ability::Strength).unwrap());
        assert_eq!(7, abilities.get_cost(Ability::Dexterity).unwrap());
        assert_eq!(4, abilities.get_cost(Ability::Constitution).unwrap());
        assert_eq!(12, abilities.get_cost(Ability::Intelligence).unwrap());
        assert_eq!(5, abilities.get_cost(Ability::Wisdom).unwrap());
        assert_eq!(2, abilities.get_cost(Ability::Charisma).unwrap());
    }

    #[test]
    fn test_character_abilities_set() {
        let mut abilities = create_test_character_abilities();
        abilities.set(Ability::Strength, 10);
        assert_eq!(10, abilities.get(Ability::Strength).unwrap().score);
    }


    #[test]
    fn test_character_abilities_total_cost() {
        let abilities = create_test_character_abilities();
        let actual = abilities.total_cost();
        // @formatter:off
        assert_eq!(actual,
           0  +  // str
           7  +  // dex
           4  +  // con
           12 +  // int
           5  +  // wis
           2     // cha
        );
    }
}