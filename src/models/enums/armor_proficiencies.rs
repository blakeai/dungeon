use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum ArmorProficiency {
    All,
    Light,
    Medium,
    None,
    Shields,
}
