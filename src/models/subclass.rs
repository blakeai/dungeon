use derive_more::Constructor;
use crate::models::character_class::CharacterClass;

#[derive(Constructor, Debug)]
pub(crate) struct Subclass {
    pub(crate) name: String,
    pub(crate) parent_class: CharacterClass,
    pub(crate) level_requirements: u8,
    pub(crate) features: Vec<String>,
}
