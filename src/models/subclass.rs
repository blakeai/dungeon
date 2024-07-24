use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::enums::classes::ClassType;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Subclass {
    pub name: String,
    pub parent: ClassType,
    pub levels: HashMap<String, Vec<String>>,
}

#[cfg(test)]
pub(super) mod tests {
    use maplit::hashmap;

    use crate::models::enums::classes::ClassType;

    use super::*;

    pub(crate) fn create_test_subclass(parent: ClassType) -> Subclass {
        Subclass {
            name: "The Fiend".to_string(),
            parent,
            levels: hashmap! {
                "1".to_string() => vec!["temp hit points on kill".to_string()],
                "6".to_string() => vec!["add d10 to ability check".to_string()],
                "10".to_string() => vec!["resistant to 1 damage type".to_string(),"swappable".to_string()],
            },
        }
    }

    #[test]
    fn test_create_test_subclass() {
        let subclass = create_test_subclass(ClassType::Warlock);
        assert_eq!(subclass.name, "The Fiend");
        assert_eq!(subclass.parent, ClassType::Warlock);
        assert_eq!(
            &vec!["temp hit points on kill".to_string()],
            subclass.levels.get("1").unwrap()
        );
        assert_eq!(
            &vec!["add d10 to ability check".to_string()],
            subclass.levels.get("6").unwrap()
        );
        assert_eq!(
            &vec!["resistant to 1 damage type".to_string(), "swappable".to_string()],
            subclass.levels.get("10").unwrap()
        );
    }
}