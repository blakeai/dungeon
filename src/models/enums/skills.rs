use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Display)]
pub enum Skill {
    #[serde(alias = "any")]
    Any,
    #[serde(alias = "acrobatics")]
    Acrobatics,
    #[serde(alias = "arcana")]
    Arcana,
    #[serde(alias = "athletics")]
    Athletics,
    #[serde(alias = "animal handling")]
    AnimalHandling,
    #[serde(alias = "deception")]
    Deception,
    #[serde(alias = "history")]
    History,
    #[serde(alias = "insight")]
    Insight,
    #[serde(alias = "intimidation")]
    Intimidation,
    #[serde(alias = "investigation")]
    Investigation,
    #[serde(alias = "medicine")]
    Medicine,
    #[serde(alias = "musical instrument")]
    MusicalInstrument,
    #[serde(alias = "nature")]
    Nature,
    #[serde(alias = "perception")]
    Perception,
    #[serde(alias = "performance")]
    Performance,
    #[serde(alias = "persuasion")]
    Persuasion,
    #[serde(alias = "religion")]
    Religion,
    #[serde(alias = "sleight of hand")]
    SleightOfHand,
    #[serde(alias = "stealth")]
    Stealth,
    #[serde(alias = "survival")]
    Survival,
}