use std::str::FromStr;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Display)]
pub enum ArmorProficiency {
    All,
    Light,
    Medium,
    None,
    Shields,
}

impl FromStr for ArmorProficiency {
    type Err = ();

    fn from_str(input: &str) -> Result<ArmorProficiency, Self::Err> {
        match input {
            "all" => Ok(ArmorProficiency::All),
            "light" => Ok(ArmorProficiency::Light),
            "medium" => Ok(ArmorProficiency::Medium),
            "none" => Ok(ArmorProficiency::None),
            "shields" => Ok(ArmorProficiency::Shields),
            _ => Err(()),
        }
    }
}