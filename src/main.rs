#[macro_use]
extern crate lazy_static;

use maplit::hashmap;

use models::{
    ability_score::AbilityScore,
    ability_type::AbilityType,
    attributes::Attributes,
    cantrip::Cantrip,
    character::Character,
    character_abilities::CharacterAbilities,
    character_class::CharacterClass,
    race::Race,
    spell::Spell,
    subclass::Subclass,
};

mod models;
mod engine;

fn main() {
    // Define race
    let elf_race = Race::Elf;

    // Define character class
    let character_class = CharacterClass {
        name: "Wizard".to_string(),
        hit_die: 6,
    };

    // Define subclass
    let subclass = Subclass {
        name: "Evocation".to_string(),
        parent_class: character_class.clone(),
        level_requirements: 2,
        features: vec!["Sculpt Spells".to_string()],
    };

    // Define abilities using a HashMap
    let character_abilities = CharacterAbilities::new(hashmap! {
        AbilityType::Strength => AbilityScore::new(8, 0),
        AbilityType::Dexterity => AbilityScore::new(14, 2),
        AbilityType::Constitution => AbilityScore::new(12, 4),
        AbilityType::Intelligence => AbilityScore::new(16, 3),
        AbilityType::Wisdom => AbilityScore::new(13, 1),
        AbilityType::Charisma => AbilityScore::new(10, 0),
    });

    // Get the total cost of abilities
    println!("Total ability cost: {}", character_abilities.total_cost());

    // Define attributes
    let attributes = Attributes {
        armor_class: 12,
        hit_points: 20,
        speed: 30,
    };

    // Define cantrips
    let cantrips = vec![
        Cantrip {
            name: "Fire Bolt".to_string(),
            description: "A bolt of fire".to_string(),
        },
    ];

    // Define spells
    let spells = vec![
        Spell {
            name: "Magic Missile".to_string(),
            level: 1,
            description: "A missile of magical energy".to_string(),
        },
    ];

    // Build the character
    let character = Character {
        name: "Gandalf".to_string(),
        race: elf_race,
        character_class: character_class.clone(),
        subclass: Some(subclass),
        level: 5,
        abilities: character_abilities,
        attributes,
        cantrips,
        spells,
    };

    println!("{:#?}", character);
}
