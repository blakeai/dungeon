use derive_more::Constructor;
use std::fs;
use std::collections::HashMap;

use crate::utils::env_utils::load_filename;

#[derive(Constructor, Debug)]
pub struct AbilityScore {
    pub score: i32,
    pub cost: i32,
}

#[derive(Constructor, Debug)]
struct PointBuySystem {
    pub config: Vec<i32>,
}

lazy_static! {
    pub(crate) static ref ABILITY_COST_TABLE: Vec<AbilityScore> =
    create_cost_table()
    .expect("Failed to create ability score cost table");
}

impl AbilityScore {
    pub fn from_score(score: i32) -> Self {
        let cost = AbilityScore::get_cost(score);
        Self::new(score, cost)
    }

    fn get_cost(score: i32) -> i32 {
        ABILITY_COST_TABLE.iter()
            .find(|a| a.score == score)
            .map(|a| a.cost)
            .unwrap_or(0)
    }
}

fn create_cost_table() -> Result<Vec<AbilityScore>, Box<dyn std::error::Error>> {
    let path = load_filename("POINT_BUY_CONFIG_FILENAME".to_string());
    let raw_data = fs::read_to_string(path)?;
    let data_map: HashMap<String, i32> = serde_json::from_str(&raw_data)?;
    let point_buy_system: Vec<i32> = (1..=19)
        .filter_map(|i| data_map.get(&i.to_string()).cloned())
        .collect();
    let ability_scores = point_buy_system
        .iter().enumerate()
        .map(|(index, &cost)| AbilityScore::new(8 + index as i32, cost))
        .collect();
    return Ok(ability_scores);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ability_score_from_score() {
        let ability_score = AbilityScore::from_score(8);
        assert_eq!(ability_score.score, 8);
        assert_eq!(ability_score.cost, 0);
    }

    #[test]
    fn test_ability_score_from_score_15() {
        let ability_score = AbilityScore::from_score(15);
        assert_eq!(ability_score.score, 15);
        assert_eq!(ability_score.cost, 12);
    }
}

