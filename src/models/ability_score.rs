use derive_more::Constructor;
use crate::models::point_buy_system::PointBuySystem;

use crate::utils::env_utils::load_filename;

#[derive(Constructor, Debug)]
pub struct AbilityScore {
    pub score: i32,
    pub cost: i32,
}

// Define the cost table based on the point-buy system
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
    let point_buy_system = PointBuySystem::load_from_json(&path)?;
    let ability_scores = point_buy_system.config
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