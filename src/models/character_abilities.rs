use super::{ability_score::AbilityScore, ability_type::AbilityType};
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