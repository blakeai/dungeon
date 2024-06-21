use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Display)]
pub enum WeaponProficiency {
    #[serde(alias = "all")]
    All,
    #[serde(alias = "clubs")]
    Clubs,
    #[serde(alias = "daggers")]
    Daggers,
    #[serde(alias = "hand crossbows")]
    HandCrossbows,
    #[serde(alias = "javelins")]
    Javelins,
    #[serde(alias = "light crossbows")]
    LightCrossbows,
    #[serde(alias = "longswords")]
    Longswords,
    #[serde(alias = "maces")]
    Maces,
    #[serde(alias = "morningstars")]
    Morningstars,
    #[serde(alias = "quarterstaves")]
    Quarterstaves,
    #[serde(alias = "rapiers")]
    Rapiers,
    #[serde(alias = "scimitars")]
    Scimitars,
    #[serde(alias = "shortswords")]
    Shortswords,
    #[serde(alias = "sickles")]
    Sickles,
    #[serde(alias = "simple")]
    Simple,
    #[serde(alias = "spears")]
    Spears,
}