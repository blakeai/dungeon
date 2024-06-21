use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Display)]
pub enum ArmorProficiency {
    #[serde(alias = "all")]
    All,
    #[serde(alias = "light")]
    Light,
    #[serde(alias = "medium")]
    Medium,
    #[serde(alias = "heavy")]
    Heavy,
    #[serde(alias = "none")]
    None,
    #[serde(alias = "shields")]
    Shields,
}
