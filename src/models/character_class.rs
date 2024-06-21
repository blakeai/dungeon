use std::collections::HashMap;
use crate::models::enums::armor_proficiencies::ArmorProficiency;
use crate::models::enums::classes::ClassType;
use crate::models::enums::skills::Skill;
use crate::models::enums::weapon_proficiencies::WeaponProficiency;


#[derive(Debug)]
pub struct CharacterClass {
    pub class_type: ClassType,
    pub subclass_level: u8,
    pub weapon_proficiency: Vec<WeaponProficiency>,
    pub armor_proficiency: Vec<ArmorProficiency>,
    pub skills: HashMap<Skill, u8>,
    pub levels: HashMap<u8, Vec<String>>,
}

#[cfg(test)]
pub(super) mod tests {
    use maplit::hashmap;
    use super::*;

    pub(crate) fn create_test_character_class() -> CharacterClass {
        CharacterClass {
            class_type: ClassType::Wizard,
            weapon_proficiency: vec![WeaponProficiency::Simple],
            armor_proficiency: vec![ArmorProficiency::None],
            skills: hashmap! {
                Skill::Arcana => 2,
                Skill::History => 1,
                Skill::Insight => 1,
                Skill::Investigation => 1,
                Skill::Medicine => 1,
                Skill::Religion => 1,
            },
            subclass_level: 2,
            levels: hashmap! {
                1 => vec![
                    "level 1 spells (int)".to_string(),
                    "cantrips".to_string(), 
                    "arcane recovery charges to regen spell slot".to_string()
                ],
                2 => vec![
                    "subclass".to_string(),
                    "learn subclass scrolls for cheap".to_string()
                ],
                3 => vec![
                    "level 2 spells".to_string()
                ],
                4 => vec![
                    "feat".to_string(),
                    "cantrip".to_string()
                ],
                5  => vec!["level 3 spells".to_string()],
                7  => vec!["level 4 spells".to_string()],
                8  => vec!["feat".to_string()],
                9  => vec!["level 5 spells".to_string()],
                10 => vec!["cantrip".to_string()],
                11 => vec!["level 6 spells".to_string()],
                12 => vec!["feat".to_string()],
            },
        }
    }

    #[test]
    fn test_create_test_character_class() {
        let character_class = create_test_character_class();
        assert_eq!(character_class.class_type, ClassType::Wizard);
        assert_eq!(character_class.weapon_proficiency.len(), 1);
        assert_eq!(character_class.armor_proficiency.len(), 1);
        assert_eq!(character_class.skills.len(), 6);
        assert_eq!(character_class.subclass_level, 2);
        assert_eq!(character_class.levels.len(), 11);
    }
}