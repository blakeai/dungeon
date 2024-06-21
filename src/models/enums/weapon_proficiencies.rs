use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Display)]
pub enum WeaponProficiency {
    #[serde(alias = "all")]             All,
    #[serde(alias = "clubs")]           Clubs,
    #[serde(alias = "maces")]           Maces,
    #[serde(alias = "simple")]          Simple,
    #[serde(alias = "spears")]          Spears,
    #[serde(alias = "daggers")]         Daggers,
    #[serde(alias = "rapiers")]         Rapiers,
    #[serde(alias = "sickles")]         Sickles,
    #[serde(alias = "javelins")]        Javelins,
    #[serde(alias = "scimitars")]       Scimitars,
    #[serde(alias = "longswords")]      Longswords,
    #[serde(alias = "shortswords")]     Shortswords,
    #[serde(alias = "morningstars")]    Morningstars,
    #[serde(alias = "quarterstaves")]   Quarterstaves,
    #[serde(alias = "hand crossbows")]  HandCrossbows,
    #[serde(alias = "light crossbows")] LightCrossbows,
}
