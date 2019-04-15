/// dead module
pub struct ROM {
    pub(crate) program: Program,
    pub(crate) character: Character,
}

pub(crate) type Program = Vec<u8>;
pub(crate) type Character = Vec<u8>;
