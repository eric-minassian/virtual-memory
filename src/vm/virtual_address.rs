use crate::{
    constants::{PAGE_SIZE_BITS, SEGMENT_SIZE_BITS},
    error::{VMError, VMResult},
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct VirtualAddress {
    pub s: u16,
    pub p: u16,
    pub w: u16,
    pub pw: u32,
}

impl VirtualAddress {
    /// Creates a new `VirtualAddress` from the given `virtual_address`.
    ///
    /// # Errors
    /// - `VMError::VirtualAddressLeadingBits` if the leading bits of the `virtual_address` are not 0.
    pub const fn new(virtual_address: u32) -> VMResult<Self> {
        let mask = u32::MAX >> (32 - (SEGMENT_SIZE_BITS + PAGE_SIZE_BITS + PAGE_SIZE_BITS));

        if (virtual_address & mask) != virtual_address {
            return Err(VMError::VirtualAddressLeadingBits);
        }

        Ok(Self {
            s: (virtual_address >> 18) as u16,
            w: (virtual_address & 0x1FF) as u16,
            p: ((virtual_address >> 9) & 0x1FF) as u16,
            pw: virtual_address & 0x3FFFF,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_virtual_address() {
        let virtual_address = VirtualAddress::new(0);
        assert_eq!(
            virtual_address,
            Ok(VirtualAddress {
                s: 0,
                p: 0,
                w: 0,
                pw: 0
            })
        );

        let virtual_address = VirtualAddress::new(0x3FFFF);
        assert_eq!(
            virtual_address,
            Ok(VirtualAddress {
                s: 0,
                p: 0x1FF,
                w: 0x1FF,
                pw: 0x3FFFF
            })
        );

        let virtual_address = VirtualAddress::new(0x7FFFFFF);
        assert_eq!(
            virtual_address,
            Ok(VirtualAddress {
                s: 0x1FF,
                p: 0x1FF,
                w: 0x1FF,
                pw: 0x3FFFF
            })
        );
    }

    #[test]
    fn new_virtual_address_invalid() {
        let virtual_address = VirtualAddress::new(0x80000000);
        assert_eq!(virtual_address, Err(VMError::VirtualAddressLeadingBits));
    }
}
