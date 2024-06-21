use serde::{Deserialize, Serialize};
use crate::models::character_class::CharacterClass;
use crate::models::subclass::Subclass;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClassContainer {
    pub class: CharacterClass,
    pub subclasses: Vec<Subclass>,
}
