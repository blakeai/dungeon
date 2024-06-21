use std::str::FromStr;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Display)]
pub enum Skill {
    Any,
    Acrobatics,
    Arcana,
    Athletics,
    AnimalHandling,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    MusicalInstrument,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl FromStr for Skill {
    type Err = ();

    fn from_str(input: &str) -> Result<Skill, Self::Err> {
        match input {
            "any" => Ok(Skill::Any),
            "acrobatics" => Ok(Skill::Acrobatics),
            "arcana" => Ok(Skill::Arcana),
            "athletics" => Ok(Skill::Athletics),
            "animal handling" => Ok(Skill::AnimalHandling),
            "deception" => Ok(Skill::Deception),
            "history" => Ok(Skill::History),
            "insight" => Ok(Skill::Insight),
            "intimidation" => Ok(Skill::Intimidation),
            "investigation" => Ok(Skill::Investigation),
            "medicine" => Ok(Skill::Medicine),
            "musical instrument" => Ok(Skill::MusicalInstrument),
            "nature" => Ok(Skill::Nature),
            "perception" => Ok(Skill::Perception),
            "performance" => Ok(Skill::Performance),
            "persuasion" => Ok(Skill::Persuasion),
            "religion" => Ok(Skill::Religion),
            "sleight of hand" => Ok(Skill::SleightOfHand),
            "stealth" => Ok(Skill::Stealth),
            "survival" => Ok(Skill::Survival),
            _ => Err(()),
        }
    }
}
