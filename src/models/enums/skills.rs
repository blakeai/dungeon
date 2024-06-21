use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Display)]
pub enum Skill {
    #[serde(alias = "any")]                Any,
    #[serde(alias = "arcana")]             Arcana,
    #[serde(alias = "nature")]             Nature,
    #[serde(alias = "history")]            History,
    #[serde(alias = "insight")]            Insight,
    #[serde(alias = "stealth")]            Stealth,
    #[serde(alias = "medicine")]           Medicine,
    #[serde(alias = "survival")]           Survival,
    #[serde(alias = "religion")]           Religion,
    #[serde(alias = "athletics")]          Athletics,
    #[serde(alias = "deception")]          Deception,
    #[serde(alias = "acrobatics")]         Acrobatics,
    #[serde(alias = "perception")]         Perception,
    #[serde(alias = "persuasion")]         Persuasion,
    #[serde(alias = "performance")]        Performance,
    #[serde(alias = "intimidation")]       Intimidation,
    #[serde(alias = "investigation")]      Investigation,
    #[serde(alias = "sleight of hand")]    SleightOfHand,
    #[serde(alias = "animal handling")]    AnimalHandling,
    #[serde(alias = "musical instrument")] MusicalInstrument,
}