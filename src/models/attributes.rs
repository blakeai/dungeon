#[derive(Debug)]
pub struct Attributes {
    pub armor_class: u8,
    pub hit_points: i32,
    pub speed: u8,
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    pub(crate) fn create_test_attributes() -> Attributes {
        Attributes {
            armor_class: 20,
            hit_points: 54,
            speed: 30,
        }
    }

    #[test]
    fn test_create_test_attributes() {
        let attributes = create_test_attributes();
        assert_eq!(attributes.armor_class, 20);
        assert_eq!(attributes.hit_points, 54);
        assert_eq!(attributes.speed, 30);
    }
}