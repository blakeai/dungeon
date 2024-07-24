#[allow(dead_code)]
#[derive(Debug)]
pub struct Spell {
    pub name: String,
    pub level: u8,
    pub description: String,
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    pub(crate) fn create_test_spells() -> Vec<Spell> {
        vec![
            Spell {
                name: "Magic Missile".to_string(),
                level: 1,
                description: "A missile of magic".to_string(),
            },
            Spell {
                name: "Lightning Bolt".to_string(),
                level: 3,
                description: "A bolt of lightning".to_string(),
            },
        ]
    }

    #[test]
    fn test_create_test_spells() {
        let spells = create_test_spells();
        assert_eq!(spells.len(), 2);
        let spell = &spells[0];
        assert_eq!("Magic Missile", spell.name);
        assert_eq!(1, spell.level);
        assert_eq!("A missile of magic", spell.description);
    }
}