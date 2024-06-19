#[derive(Debug, Clone)]
pub struct CharacterClass {
    pub name: String,
    pub hit_die: u8,
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    pub(crate) fn create_test_character_class() -> CharacterClass {
        CharacterClass {
            name: "Wizard".to_string(),
            hit_die: 6,
        }
    }

    #[test]
    fn test_create_test_character_class() {
        let class = create_test_character_class();
        assert_eq!(class.name, "Wizard");
        assert_eq!(class.hit_die, 6);
    }
}