#[derive(Debug, PartialEq, Eq)]
pub struct STInput {
    pub s: i32,
    pub z: i32,
    pub f: i32,
}

impl STInput {
    #[must_use]
    pub const fn new(s: i32, z: i32, f: i32) -> Self {
        Self { s, z, f }
    }
}
