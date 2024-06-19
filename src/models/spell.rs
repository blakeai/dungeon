use derive_more::Constructor;

#[derive(Constructor, Debug)]
pub(crate) struct Spell {
    pub(crate) name: String,
    pub(crate) level: u8,
    pub(crate) description: String,
}
