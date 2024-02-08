#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct VirtualAddress {
    pub s: u16,
    pub p: u16,
    pub w: u16,
    pub pw: u32,
}

impl VirtualAddress {
    pub const fn new(virtual_address: u32) -> Self {
        Self {
            s: (virtual_address >> 18) as u16,
            w: (virtual_address & 0x1FF) as u16,
            p: ((virtual_address >> 9) & 0x1FF) as u16,
            pw: virtual_address & 0x3FFFF,
        }
    }
}
