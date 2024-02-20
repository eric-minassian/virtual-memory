use crate::constants::PAGE_SIZE;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Frame {
    pub free: bool,
    pub data: [i32; PAGE_SIZE],
}

impl Frame {
    pub const fn new() -> Self {
        Self {
            free: true,
            data: [0; PAGE_SIZE],
        }
    }
}
