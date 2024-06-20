use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
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
