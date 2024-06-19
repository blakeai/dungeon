use derive_more::Constructor;

#[derive(Constructor, Debug)]
pub struct AbilityScore {
    pub score: i32,
    pub cost: i32,
}

// Define the cost table based on the point-buy system
lazy_static! {
    pub static ref ABILITY_COST_TABLE: Vec<AbilityScore> = vec![
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