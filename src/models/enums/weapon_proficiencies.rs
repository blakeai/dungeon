use std::str::FromStr;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Display)]
pub enum WeaponProficiency {
    All,
    Clubs,
    Daggers,
    HandCrossbows,
    Javelins,
    LightCrossbows,
    Longswords,
    Maces,
    Morningstars,
    Quarterstaves,
    Rapiers,
    Scimitars,
    Shortswords,
    Sickles,
    Simple,
    Spears,
}

impl FromStr for WeaponProficiency {
    type Err = ();

    fn from_str(input: &str) -> Result<WeaponProficiency, Self::Err> {
        match input {
            "all" => Ok(WeaponProficiency::All),
            "clubs" => Ok(WeaponProficiency::Clubs),
            "daggers" => Ok(WeaponProficiency::Daggers),
            "hand crossbows" => Ok(WeaponProficiency::HandCrossbows),
            "javelins" => Ok(WeaponProficiency::Javelins),
            "light crossbows" => Ok(WeaponProficiency::LightCrossbows),
            "longswords" => Ok(WeaponProficiency::Longswords),
            "maces" => Ok(WeaponProficiency::Maces),
            "morningstars" => Ok(WeaponProficiency::Morningstars),
            "quarterstaves" => Ok(WeaponProficiency::Quarterstaves),
            "rapiers" => Ok(WeaponProficiency::Rapiers),
            "scimitars" => Ok(WeaponProficiency::Scimitars),
            "shortswords" => Ok(WeaponProficiency::Shortswords),
            "sickles" => Ok(WeaponProficiency::Sickles),
            "simple" => Ok(WeaponProficiency::Simple),
            "spears" => Ok(WeaponProficiency::Spears),
            _ => Err(()),
        }
    }
}