use crate::models::attributes::Attributes;
use crate::models::cantrip::Cantrip;
use crate::models::character_abilities::CharacterAbilities;
use crate::models::character_class::CharacterClass;
use crate::models::enums::races::Race;
use crate::models::spell::Spell;
use crate::models::subclass::Subclass;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub character_class: CharacterClass,
    pub subclass: Option<Subclass>,
    pub level: u8,
    pub abilities: CharacterAbilities,
    pub attributes: Attributes,
    pub cantrips: Vec<Cantrip>,
    pub spells: Vec<Spell>,
}

#[cfg(test)]
mod tests {
    use crate::models::attributes::tests::create_test_attributes;
    use crate::models::cantrip::tests::create_test_cantrips;
    use crate::models::character_abilities::tests::create_test_character_abilities;
    use crate::models::character_class::tests::create_test_character_class;
    use crate::models::enums::abilities::Ability;
    use crate::models::spell::tests::create_test_spells;
    use crate::models::subclass::tests::create_test_subclass;

    use super::*;

    pub(crate) fn create_test_character() -> Character {
        let class = create_test_character_class();
        let subclass = create_test_subclass(&class);
        Character {
            name: "Gandalf".to_string(),
            race: Race::Elf,
            character_class: class,
            subclass: Some(subclass),
            level: 12,
            abilities: create_test_character_abilities(),
            attributes: create_test_attributes(),
            cantrips: create_test_cantrips(),
            spells: create_test_spells(),
        }
    }

    #[test]
    fn test_create_test_character() {
        let character = create_test_character();
        assert_eq!(character.name, "Gandalf");
        assert_eq!(character.race.as_ref(), "Elf");
        assert_eq!(character.character_class.name, "Wizard");
        assert_eq!(character.subclass.as_ref().unwrap().name, "Evocation");
        assert_eq!(character.level, 12);
        assert_eq!(character.abilities.get(Ability::Strength).unwrap().score, 8);
        assert_eq!(character.attributes.armor_class, 20);
        assert_eq!(character.cantrips.len(), 1);
        assert_eq!(character.spells.len(), 2);
    }
}