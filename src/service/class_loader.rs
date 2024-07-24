use std::collections::HashMap;
use std::error;
use std::fs;
use std::path::Path;

use crate::config::environment::EnvVar;
use crate::models::class_subclass_container::ClassContainer;
use crate::models::enums::classes::ClassType;
use crate::utils::env_utils::getenv;
use tracing::{debug, info};

pub fn extract_classes() -> Result<HashMap<ClassType, ClassContainer>, Box<dyn error::Error>> {
    let filename = getenv(EnvVar::CharacterClassConfigFilename);
    let path = Path::new(&filename).to_str().unwrap();
    let raw = fs::read_to_string(&path)?;


    let result = serde_json::from_str(&raw);
    let character_map: HashMap<ClassType, ClassContainer> = result.unwrap();

    info!("Deserialized character classes from file: {}", &path);
    debug!("{:#?}", character_map);

    Ok(character_map)
}