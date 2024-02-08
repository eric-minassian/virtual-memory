#[derive(Debug, PartialEq, Eq)]
pub struct PTInput {
    pub s: i32,
    pub p: i32,
    pub f: i32,
}

impl PTInput {
    #[must_use]
    pub const fn new(s: i32, p: i32, f: i32) -> Self {
        Self { s, p, f }
    }
}
