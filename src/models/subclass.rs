use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::enums::classes::ClassType;
use crate::utils::deser_utils::deserialize_comma_separated_to_string_map;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Subclass {
    pub name: String,
    pub parent: ClassType,
    #[serde(deserialize_with = "deserialize_comma_separated_to_string_map")]
    pub levels: HashMap<u8, Vec<String>>,
}

#[cfg(test)]
pub(super) mod tests {
    use maplit::hashmap;

    use crate::models::character_class::tests::create_test_character_class;
    use crate::models::enums::classes::ClassType;

    use super::*;

    pub(crate) fn create_test_subclass(parent: ClassType) -> Subclass {
        Subclass {
            name: "The Fiend".to_string(),
            parent,
            levels: hashmap! {
                1 => vec!["temp hit points on kill".to_string()],
                6 => vec!["add d10 to ability check".to_string()],
                10 => vec!["resistant to 1 damage type".to_string(),"swappable".to_string()],
            },
        }
    }

    #[test]
    fn test_create_test_subclass() {
        let parent_class = create_test_character_class();
        let subclass = create_test_subclass(ClassType::Warlock);
        assert_eq!(subclass.name, "The Fiend");
        assert_eq!(subclass.parent, ClassType::Warlock);
        assert_eq!(
            &vec!["temp hit points on kill".to_string()],
            subclass.levels.get(&1).unwrap()
        );
        assert_eq!(
            &vec!["add d10 to ability check".to_string()],
            subclass.levels.get(&6).unwrap()
        );
        assert_eq!(
            &vec!["resistant to 1 damage type".to_string(), "swappable".to_string()],
            subclass.levels.get(&10).unwrap()
        );
    }
}