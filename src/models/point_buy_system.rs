use std::collections::HashMap;
use std::fs;

use derive_more::Constructor;

#[derive(Constructor, Debug)]
pub struct PointBuySystem {
    //array of i32
    pub config: Vec<i32>,
}

impl PointBuySystem {
    pub fn load_from_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path)?;
        let as_map: HashMap<String, i32> = serde_json::from_str(&data)?;

        let config_vec: Vec<i32> = (1..=19)
            .filter_map(|i| as_map.get(&i.to_string()).cloned())
            .collect();

        Ok(Self::new(config_vec))
    }
}