use derive_more::Constructor;
use crate::models::{
    attributes::Attributes,
    cantrip::Cantrip,
    character_abilities::CharacterAbilities,
    character_class::CharacterClass,
    race::Race,
    spell::Spell,
    subclass::Subclass,
};

#[derive(Constructor, Debug)]
pub struct Character {
    pub(crate) name: String,
    pub(crate) race: Race,
    pub(crate) character_class: CharacterClass,
    pub(crate) subclass: Option<Subclass>,
    pub(crate) level: u8,
    pub(crate) abilities: CharacterAbilities,
    pub(crate) attributes: Attributes,
    pub(crate) cantrips: Vec<Cantrip>,
    pub(crate) spells: Vec<Spell>,
}