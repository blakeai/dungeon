#[derive(Debug)]
pub struct Cantrip {
    pub name: String,
    pub description: String,
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    pub(crate) fn create_test_cantrips() -> Vec<Cantrip> {
        vec![
            Cantrip {
                name: "Fire Bolt".to_string(),
                description: "A bolt of fire".to_string(),
            },
        ]
    }

    #[test]
    fn test_create_test_cantrips() {
        let cantrips = create_test_cantrips();
        assert_eq!(cantrips.len(), 1);
        let cantrip = &cantrips[0];
        assert_eq!(cantrip.name, "Fire Bolt");
        assert_eq!(cantrip.description, "A bolt of fire");
    }
}