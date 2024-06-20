use crate::models::character_class::CharacterClass;

#[derive(Debug)]
pub struct Subclass {
    pub name: String,
    pub parent_class: CharacterClass,
    pub level_requirements: u8,
    pub features: Vec<String>,
}

#[cfg(test)]
pub(super) mod tests {
    use crate::models::character_class::tests::create_test_character_class;

    use super::*;

    pub(crate) fn create_test_subclass(class: &CharacterClass) -> Subclass {
        Subclass {
            name: "Evocation".to_string(),
            parent_class: class.clone(),
            level_requirements: 2,
            features: vec!["Sculpt Spells".to_string()],
        }
    }

    #[test]
    fn test_create_test_subclass() {
        let parent_class = create_test_character_class();
        let subclass = create_test_subclass(&parent_class);
        assert_eq!(subclass.name, "Evocation");
        assert_eq!(subclass.parent_class.name, "Wizard");
        assert_eq!(subclass.level_requirements, 2);
        assert_eq!(subclass.features.len(), 1);
    }
}