use std::collections::HashMap;
use std::error;
use std::fs;
use std::path::Path;

use crate::config::environment::EnvVar;
use crate::models::class_subclass_container::ClassContainer;
use crate::utils::env_utils::getenv;

pub fn extract_classes() -> Result<Vec<ClassContainer>, Box<dyn error::Error>> {
    
    let filename = getenv(EnvVar::CharacterClassConfigFilename);
    let path = Path::new(&filename).to_str().unwrap();
    let raw = fs::read_to_string(path)?;


    let result = serde_json::from_str(&raw);
    let character_map: HashMap<String, ClassContainer> = result.unwrap();

    // Convert the HashMap values into a Vec<CharacterClass>
    let character_classes: Vec<ClassContainer> = character_map.into_values().collect();

    // Print the deserialized data
    println!("{:#?}", character_classes);
    Ok(character_classes)
}