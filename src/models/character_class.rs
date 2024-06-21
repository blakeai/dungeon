use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::models::enums::abilities::Ability;

use crate::models::enums::armor_proficiencies::ArmorProficiency;
use crate::models::enums::classes::ClassType;
use crate::models::enums::skills::Skill;
use crate::models::enums::weapon_proficiencies::WeaponProficiency;

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterClass {
    pub name: ClassType,
    pub subclass_level: u8,
    pub weapon_proficiency: Vec<WeaponProficiency>,
    pub armor_proficiency: Vec<ArmorProficiency>,
    pub saving_throws: Vec<Ability>,
    pub skills: Vec<Skill>,
    pub levels: HashMap<String, Vec<String>>,
}

#[cfg(test)]
pub(super) mod tests {
    use maplit::hashmap;

    use super::*;

    pub(crate) fn create_test_character_class() -> CharacterClass {
        CharacterClass {
            name: ClassType::Wizard,
            weapon_proficiency: vec![WeaponProficiency::Simple],
            armor_proficiency: vec![ArmorProficiency::None],
            saving_throws: vec![Ability::Intelligence, Ability::Wisdom],
            skills: vec![
                Skill::Arcana,
                Skill::Arcana,
                Skill::History,
                Skill::Insight,
                Skill::Investigation,
                Skill::Medicine,
                Skill::Religion,
            ],
            subclass_level: 2,
            levels: hashmap! {
                "1".to_string() => vec![
                    "level 1 spells (int)".to_string(),
                    "cantrips".to_string(), 
                    "arcane recovery charges to regen spell slot".to_string()
                ],
                "2".to_string() => vec![
                    "subclass".to_string(),
                    "learn subclass scrolls for cheap".to_string()
                ],
                "3".to_string() => vec![
                    "level 2 spells".to_string()
                ],
                "4".to_string() => vec![
                    "feat".to_string(),
                    "cantrip".to_string()
                ],
                "5".to_string()  => vec!["level 3 spells".to_string()],
                "7".to_string()  => vec!["level 4 spells".to_string()],
                "8".to_string()  => vec!["feat".to_string()],
                "9".to_string()  => vec!["level 5 spells".to_string()],
                "10".to_string() => vec!["cantrip".to_string()],
                "11".to_string() => vec!["level 6 spells".to_string()],
                "12".to_string() => vec!["feat".to_string()],
            },
        }
    }

    #[test]
    fn test_create_test_character_class() {
        let character_class = create_test_character_class();
        assert_eq!(character_class.name, ClassType::Wizard);
        assert_eq!(1, character_class.weapon_proficiency.len());
        assert_eq!(1, character_class.armor_proficiency.len());
        assert_eq!(2, character_class.saving_throws.len());
        assert_eq!(7, character_class.skills.len());
        assert_eq!(2, character_class.subclass_level);
        assert_eq!(11, character_class.levels.len());
    }
}