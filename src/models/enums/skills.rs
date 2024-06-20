use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
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