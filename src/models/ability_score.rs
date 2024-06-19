use derive_more::Constructor;

#[derive(Constructor, Debug)]
pub struct AbilityScore {
    pub score: i32,
    pub cost: i32,
}

// Define the cost table based on the point-buy system
lazy_static! {
    pub(crate) static ref ABILITY_COST_TABLE: Vec<AbilityScore> = vec![
        AbilityScore::new(8, 0),
        AbilityScore::new(9, 1),
        AbilityScore::new(10, 2),
        AbilityScore::new(11, 3),
        AbilityScore::new(12, 4),
        AbilityScore::new(13, 5),
        AbilityScore::new(14, 7),
        AbilityScore::new(15, 9),
        AbilityScore::new(16, 12), 
    ];
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
    fn test_ability_score_from_score_16() {
        let ability_score = AbilityScore::from_score(16);
        assert_eq!(ability_score.score, 16);
        assert_eq!(ability_score.cost, 12);
    }
}